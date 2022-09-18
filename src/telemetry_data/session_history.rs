use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader,

    pub car_index: u8,
    pub num_laps: u8,
    pub num_tyre_stints: u8,

    pub best_lap_time_num: u8,
    pub best_sector_1_lap_num: u8,
    pub best_sector_2_lap_num: u8,
    pub best_sector_3_lap_num: u8,

    pub lap_history_data: HistoryData,

    pub tyre_stints_history_data: [TyreStintHistoryData; 8],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct HistoryData {
    pub first_32: [LapHistoryData; 32],
    pub second_32: [LapHistoryData; 32],
    pub third_32: [LapHistoryData; 32],
    pub last_4: [LapHistoryData; 4],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,
    pub sector_1_time_in_ms: u16,
    pub sector_2_time_in_ms: u16,
    pub sector_3_time_in_ms: u16,
    pub lap_valid_bit_flags: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,
    pub tyre_actual_compound: u8,
    pub tyre_visual_compound: u8,
}
