use async_std::net::UdpSocket;
use bincode::deserialize;
use futures;
use lazy_static::__Deref;
use std::{
    ops::{Deref, Generator, GeneratorState},
    pin::Pin,
};

pub struct TelemetryLock<T>(std::sync::Arc<std::sync::Mutex<T>>);

impl<T> TelemetryLock<T> {
    pub fn new(telemetry_packet: T) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(telemetry_packet)))
    }

}


use crate::telemetry_data::{
    car_damage_data::PacketCarDamageData,
    car_setup_data::PacketCarSetupData,
    car_status_data::PacketCarStatusData,
    car_telemetry_data::PacketCarTelemetryData,
    event_data::{PacketEventData, PacketEventFinal},
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

pub async fn event_loop_generator(port: &str) -> impl Generator<Yield = F1Data, Return = ()> {
    let mut buf = vec![0; 2048];
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    move || loop {
        let (n, _peer) = futures::executor::block_on(socket.recv_from(&mut buf)).unwrap();
        let to_yield = match n {
            1464 => {
                let decoded: Result<PacketMotionData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Motion");
                }
                let decoded: PacketMotionData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());

                F1Data::Motion(decoded)
            }
            1347 => {
                let decoded: Result<PacketCarTelemetryData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Motion");
                }
                let decoded: PacketCarTelemetryData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());

                F1Data::Telemetry(decoded)
            }
            1015 => {
                let decoded: Result<PacketClassificationData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Classification");
                }
                let decoded: PacketClassificationData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());

                F1Data::Classification(decoded)
            }
            948 => {
                let decoded: Result<PacketCarDamageData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Damage");
                }
                let decoded: PacketCarDamageData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());

                F1Data::Damage(decoded)
            }
            1155 => {
                let decoded: Result<PacketSessionHistoryData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Damage");
                }
                let decoded: PacketSessionHistoryData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());

                F1Data::SessionHistory(decoded)
            }
            972 => {
                let decoded: Result<PacketLapData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Lap");
                }
                let decoded: PacketLapData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Lap(decoded)
            }
            40 => {
                let decoded: Result<PacketEventData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Event");
                }
                let decoded: PacketEventFinal = decoded.unwrap().decode().unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Event(decoded)
            }
            632 => {
                let decoded: Result<PacketSessionData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Session");
                }
                let decoded: PacketSessionData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Session(decoded)
            }
            1257 => {
                let decoded: Result<PacketParticipantData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Participant");
                }
                let decoded: PacketParticipantData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Participant(decoded)
            }
            1102 => {
                let decoded: Result<PacketCarSetupData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Setup");
                }
                let decoded: PacketCarSetupData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Setup(decoded)
            }
            2347 => {
                let decoded: Result<PacketCarTelemetryData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Telemetry");
                }
                let decoded: PacketCarTelemetryData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Telemetry(decoded)
            }
            1058 => {
                let decoded: Result<PacketCarStatusData, _> = deserialize(&buf);
                if decoded.is_err() {
                    panic!("Status");
                }
                let decoded: PacketCarStatusData = decoded.unwrap();
                // eprintln!("{}", serde_json::to_string_pretty(&decoded).unwrap());
                F1Data::Status(decoded)
            }
            _ => continue,
        };

        yield to_yield;
    }
}
