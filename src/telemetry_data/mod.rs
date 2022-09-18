pub mod car_damage_data;
pub mod car_setup_data;
pub mod car_status_data;
pub mod car_telemetry_data;
pub mod event_data;
pub mod final_classification;
pub mod lap_data;
pub mod motion_data;
pub mod packet_header;
pub mod participant_data;
pub mod session_data;
pub mod session_history;

use serde::Serialize;

#[derive(Serialize)]
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
}
