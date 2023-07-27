use async_std::net::UdpSocket;
use bincode::deserialize;
use futures;


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
    event_data::PacketEventFinal,
    final_classification::PacketClassificationData,
    lap_data::PacketLapData,
    motion_data::PacketMotionData,
    participant_data::PacketParticipantData,
    session_data::PacketSessionData,
    session_history::PacketSessionHistoryData,
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
    ($ip:expr, $port:expr, $($size:expr => ($ty:ty, $enum_variant:ident)),+) => {
        {

            let mut buf: Vec<u8> = vec![0; 2048];
            let socket = UdpSocket::bind(
                format!(
                    "{}:{}",
                    $ip,
                    $port
                )
            ).await.unwrap();

            // #[cfg(debug)]
            eprintln!("Listening on {}:{}", $ip, $port);
    
            let generator = move || 'outer: loop {
                
                let (n, _peer) = futures::executor::block_on(socket.recv_from(&mut buf)).unwrap();
    
                let to_yield = match n {
    
                    $(
                        $size => {
                            let decoded: Result<$ty, _> = deserialize(&buf);
                            
                            // Log and then ignore incorrectly parsed packet
                            if decoded.is_err() {
                                dbg!{
                                    {
                                        eprintln!(
                                            "Error parsing packet of (presumed) type {} (size {} bytes)",
                                            stringify!($ty),
                                            n
                                        );
                                    }
                                };
                                
                                continue 'outer;
                            }

                            $enum_variant(decoded.unwrap())
                        }
                    ),+
                    
                    // Log exceptional circumstance
                    other => {
                        dbg!{
                            eprintln!("Found packet of size {other}.")
                        };
                        continue 'outer;
                    } 
    
                };

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
    Motion, 
    Telemetry,
    Classification,
    Damage,
    SessionHistory,
    Lap,
    Event,
    Session,
    Participant,
    Setup,
    Status
};


pub async fn event_loop_generator(ip: &str, port: &str) -> impl Generator<Yield = F1Data, Return = ()> {

    deserialize!(
        ip,
        port,
        // Packets of size _ bytes parsed into type X and shoved in Enum Variant Y
        1464 => (PacketMotionData, Motion),
        1347 => (PacketCarTelemetryData, Telemetry),
        1015 => (PacketClassificationData, Classification),
        948  => (PacketCarDamageData, Damage),
        1155 => (PacketSessionHistoryData, SessionHistory),
        972  => (PacketLapData, Lap),
        40   => (PacketEventFinal, Event),
        632  => (PacketSessionData, Session),
        1257 => (PacketParticipantData, Participant), 
        1102 => (PacketCarSetupData, Setup),
        // 2347 => (PacketCarTelemetryData, _),
        1058 => (PacketCarStatusData, Status)
    )

}
