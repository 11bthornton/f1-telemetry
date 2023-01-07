use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct CarDamageData {
    pub tyres_wear: [f32; 4],        // Tyre wear (percentage)
    pub tyres_damage: [u8; 4],       // Tyre damage (percentage)
    pub brakes_damage: [u8; 4],      // Brakes damage (percentage)
    pub front_left_wing_damage: u8,  // Front left wing damage (percentage)
    pub front_right_wing_damage: u8, // Front right wing damage (percentage)
    pub rear_wing_damage: u8,        // Rear wing damage (percentage)
    pub floor_damage: u8,            // Floor damage (percentage)
    pub diffuser_damage: u8,         // Diffuser damage (percentage)
    pub sidepod_damage: u8,          // Sidepod damage (percentage)
    pub drs_fault: u8,               // Indicator for DRS fault, 0 = OK, 1 = fault
    pub gear_box_damage: u8,         // Gear box damage (percentage)
    pub engine_damage: u8,           // Engine damage (percentage)
    pub engine_mguh_wear: u8,        // Engine wear MGU-H (percentage)
    pub engine_es_wear: u8,          // Engine wear ES (percentage)
    pub engine_ce_wear: u8,          // Engine wear CE (percentage)
    pub engine_ice_wear: u8,         // Engine wear ICE (percentage)
    pub engine_mguk_wear: u8,        // Engine wear MGU-K (percentage)
    pub engine_tc_wear: u8,          // Engine wear TC (percentage)
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketCarDamageData {
    pub header: PacketHeader, // Header
    pub car_damage_data: [CarDamageData; 22],
}
