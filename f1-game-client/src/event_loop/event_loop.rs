use bincode::deserialize;
use std::net::UdpSocket;

use std::{
    ops::{Generator, GeneratorState},
    pin::Pin,
};

pub struct TelemetryLock<T>(std::sync::Arc<std::sync::Mutex<T>>);

impl<T> TelemetryLock<T> {
    pub fn new(telemetry_packet: T) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(telemetry_packet)))
    }
}

// Standard prelude of types
use crate::telemetry_data::{
    car_damage_data::PacketCarDamageData,
    car_setup_data::PacketCarSetupData,
    car_status_data::PacketCarStatusData,
    car_telemetry_data::PacketCarTelemetryData,
    event_data::{PacketEventData, PacketEventFinal},
    final_classification::PacketClassificationData,
    lap_data::PacketLapData,
    lobby_info::PacketLobbyInfoData,
    motion_data::PacketMotionData,
    motion_extended_data::PacketMotionExData,
    participant_data::PacketParticipantData,
    session_data::PacketSessionData,
    session_history::PacketSessionHistoryData,
    tyre_set_data::PacketTyreSetsData,
    F1Data,
};

pub struct GeneratorIteratorAdapter<G>(Pin<Box<G>>);

impl<G> GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    pub fn new(gen: G) -> Self {
        Self(Box::pin(gen))
    }
}

impl<G> Iterator for GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

macro_rules! deserialize {
    ($ip:expr, $port:expr, $($size:expr => ($ty:ty, $enum_variant:ident $(, $alt:ty)?)),+) => {
        {

            let mut buf: Vec<u8> = vec![0; 2048];
            let socket = UdpSocket::bind(
                format!(
                    "{}:{}",
                    $ip,
                    $port
                )
            ).unwrap();

            // #[cfg(debug)]
            eprintln!("Listening on {}:{}", $ip, $port);

            let generator = move || 'outer: loop {

                let (n, _peer) = socket.recv_from(&mut buf).unwrap();

                let to_yield = match n {

                    $(
                        $size => {

                            // Helps me make sure i'm not missing any data
                            // if std::mem::size_of::<$ty>() != $size as usize {

                            //     println!(
                            //         "We have been given {} bytes but struct {} is actually defined to be of size {} bytes",
                            //         $size,
                            //         stringify!($ty),
                            //         std::mem::size_of::<$ty>()
                            //     );

                            // }


                            let decoded: Result<$ty, _> = deserialize(&buf);
                            // eprintln!("{n} => {}", stringify!($ty));


                            // Log and then ignore incorrectly parsed packet
                            if decoded.is_err() {


                                eprintln!(
                                    "Error parsing packet of (presumed) type {} (size {} bytes)\n{:?}",
                                    stringify!($ty),
                                    n,
                                    decoded
                                );

                                continue 'outer;
                            }

                            let decoded = decoded.unwrap();

                            $(
                                let decoded: $alt = decoded.decode().unwrap();
                            )?

                            $enum_variant(decoded)
                        }
                    ),+

                    // Log exceptional circumstance
                    other => {
                        eprintln!("Found packet of size {other}.");
                        continue 'outer;
                    }

                };


                // println!("{to_yield:#?}");
                yield to_yield;
                // if let Some(parsed) = to_yield {
                //     yield parsed;
                // }
            };

            generator
        }
    };
}

use F1Data::{
    Classification, Damage, Event, ExtendedMotion, Lap, Lobby, Motion, Participant, Session,
    SessionHistory, Setup, Status, Telemetry, TyreSetData,
};

pub fn event_loop_generator(ip: &str, port: &str) -> impl Generator<Yield = F1Data, Return = ()> {
    deserialize!(
        ip,
        port,
        // Packets of size _ bytes parsed into type X and shoved in Enum Variant Y
        1349 => (PacketMotionData, Motion),
        1352 => (PacketCarTelemetryData, Telemetry),
        1020 => (PacketClassificationData, Classification),
        953  => (PacketCarDamageData, Damage),
        1460 => (PacketSessionHistoryData, SessionHistory),
        1131  => (PacketLapData, Lap),
        45   => (PacketEventData, Event, PacketEventFinal),
        644  => (PacketSessionData, Session),
        1306 => (PacketParticipantData, Participant),
        1107 => (PacketCarSetupData, Setup),
        1239  => (PacketCarStatusData, Status),
        1218 => (PacketLobbyInfoData, Lobby),
        217 => (PacketMotionExData, ExtendedMotion),
        231 => (PacketTyreSetsData, TyreSetData)
    )
}
