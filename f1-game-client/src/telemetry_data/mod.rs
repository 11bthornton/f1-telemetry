pub mod car_damage_data;
pub mod car_setup_data;
pub mod car_status_data;
pub mod car_telemetry_data;
pub mod event_data;
pub mod final_classification;
pub mod lap_data;
pub mod lobby_info;
pub mod motion_data;
pub mod motion_extended_data;
pub mod packet_header;
pub mod participant_data;
pub mod session_data;
pub mod session_history;
pub mod tyre_set_data;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum F1Data {
    Damage(car_damage_data::PacketCarDamageData),
    Setup(car_setup_data::PacketCarSetupData),
    Status(car_status_data::PacketCarStatusData),
    Lap(lap_data::PacketLapData),
    Motion(motion_data::PacketMotionData),
    Participant(participant_data::PacketParticipantData),
    Session(session_data::PacketSessionData),
    Event(event_data::PacketEventFinal),
    Telemetry(car_telemetry_data::PacketCarTelemetryData),
    Classification(final_classification::PacketClassificationData),
    SessionHistory(session_history::PacketSessionHistoryData),
    Lobby(lobby_info::PacketLobbyInfoData),
    ExtendedMotion(motion_extended_data::PacketMotionExData),
    TyreSetData(tyre_set_data::PacketTyreSetsData),
}

pub use car_damage_data::PacketCarDamageData;
pub use car_setup_data::PacketCarSetupData;
pub use car_status_data::PacketCarStatusData;
pub use car_telemetry_data::CarTelemetryData;
pub use event_data::PacketEventFinal as PacketEventData;
pub use final_classification::PacketClassificationData;
pub use lap_data::LapData;
pub use motion_data::PacketMotionData;
pub use packet_header::PacketHeader;
pub use participant_data::PacketParticipantData;
pub use session_data::PacketSessionData;
pub use session_history::PacketSessionHistoryData;

macro_rules! deserialize_to {
    ($type:ty) => {};
}
