use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketClassificationData {
    pub header: PacketHeader,
    pub num_cars: u8,
    pub classification_data: [FinalClassificationData; 22],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FinalClassificationData {
    pub position: u8,
    pub num_laps: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_pitstops: u8,

    pub best_lap_time_in_ms: u32,
    pub total_race_time: u64,
    pub penalties_time: u8,
    pub num_penalties: u8,
    pub num_tyre_stints: u8,
    pub tyre_stints_actual: [u8; 8],
    pub tyre_stints_visual: [u8; 8],
    pub tyre_stints_end_laps: [u8; 8],
}
