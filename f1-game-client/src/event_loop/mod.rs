pub mod event_loop;

pub use event_loop::event_loop_generator;
pub use event_loop::GeneratorIteratorAdapter;

use crate::telemetry_data;
use crate::telemetry_data::F1Data::{self};

use crate::telemetry_data::car_damage_data::PacketCarDamageData;
use crate::telemetry_data::event_data::PacketEventFinal;
use crate::telemetry_data::lap_data;
use crate::telemetry_data::lobby_info::PacketLobbyInfoData;
use crate::telemetry_data::motion_extended_data::PacketMotionExData;
use crate::telemetry_data::PacketClassificationData;
use crate::telemetry_data::PacketMotionData;

use crate::telemetry_data::participant_data::PacketParticipantData;
use crate::telemetry_data::tyre_set_data::PacketTyreSetsData;
use crate::telemetry_data::PacketCarStatusData;
use crate::telemetry_data::PacketSessionData;
use crate::telemetry_data::PacketSessionHistoryData;

pub trait DataHandler {
    #![allow(unused_variables)]

    fn on_lap_data(&self, lap_data: lap_data::PacketLapData) {}
    fn on_car_damage_data(&self, damage_data: PacketCarDamageData) {}
    fn on_car_setup_data(&self, setup_data: telemetry_data::car_setup_data::PacketCarSetupData) {}
    fn on_car_telemetry_data(
        &self,
        telemetry_data: telemetry_data::car_telemetry_data::PacketCarTelemetryData,
    ) {
    }
    fn on_event_data(&self, event_data: PacketEventFinal) {}
    fn on_participant_data(&self, participant_data: PacketParticipantData) {}
    fn on_car_status_data(&self, status_data: PacketCarStatusData) {}
    fn on_session_data(&self, session_data: PacketSessionData) {}
    fn on_session_history_data(&self, session_history_data: PacketSessionHistoryData) {}
    fn on_classification_data(&self, classification_data: PacketClassificationData) {}
    fn on_motion_data(&self, motion_data: PacketMotionData) {}
    fn on_lobby_info(&self, lobby_info_data: PacketLobbyInfoData) {}

    fn on_extended_motion_data(&self, extended_motion_data: PacketMotionExData) {}

    fn on_tyre_set_data(&self, tyre_set_data: PacketTyreSetsData) {}

    fn listen(&self, ip: &str, port: &str) {
        let generator = event_loop_generator(ip, port);

        let iter = GeneratorIteratorAdapter::new(generator);

        for packet in iter {
            match packet {
                F1Data::Damage(pack) => self.on_car_damage_data(pack),
                F1Data::Event(pack) => self.on_event_data(pack),
                F1Data::Participant(pack) => self.on_participant_data(pack),
                F1Data::Lap(pack) => self.on_lap_data(pack),
                F1Data::Telemetry(pack) => self.on_car_telemetry_data(pack),
                F1Data::Setup(pack) => self.on_car_setup_data(pack),
                F1Data::Status(pack) => self.on_car_status_data(pack),
                F1Data::Session(pack) => self.on_session_data(pack),
                F1Data::SessionHistory(pack) => self.on_session_history_data(pack),
                F1Data::Motion(pack) => self.on_motion_data(pack),
                F1Data::Classification(pack) => self.on_classification_data(pack),
                F1Data::Lobby(pack) => self.on_lobby_info(pack),
                F1Data::ExtendedMotion(pack) => self.on_extended_motion_data(pack),
                F1Data::TyreSetData(pack) => self.on_tyre_set_data(pack),
            }
        }
    }
}
