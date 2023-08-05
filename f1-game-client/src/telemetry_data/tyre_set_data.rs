use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct TyreSetData {
    actual_tyre_compound: crate::telemetry_data::car_status_data::ActualTyreCompound, // Actual tyre compound used
    visual_tyre_compound: crate::telemetry_data::car_status_data::VisualTyreCompound, // Visual tyre compound used
    wear: u8,                // Tyre wear (percentage)
    available: bool,         // Whether this set is currently available
    recommended_session: u8, // Recommended session for tyre set -- hMMM! ?!??! ?
    life_span: u8,           // Laps left in this tyre set
    usable_life: u8,         // Max number of laps recommended for this compound
    lap_delta_time: i16,     // Lap delta time in milliseconds compared to fitted set
    fitted: bool,            // Whether the set is fitted or not
}

#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct PacketTyreSetsData {
    header: PacketHeader,             // Header
    car_idx: u8,                      // Index of the car this data relates to
    tyre_set_data: [TyreSetData; 20], // 13 (dry) + 7 (wet)
    fitted_idx: u8,                   // Index into array of fitted tyre
}
