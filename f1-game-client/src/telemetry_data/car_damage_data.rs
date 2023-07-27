use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct CarDamageData {
    pub tyres: TyreData,
    pub brakes_damage: BrakeData,
    pub wing_damage: WingDamage,
    pub floor_damage: u8,
    pub diffuser_damage: u8,
    pub sidepod_damage: u8,
    pub drs_fault: u8,
    pub gear_box_damage: u8,
    pub engine_damage: u8,
    pub engine_wear: EngineWear,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TyreData {
    pub wear: [f32; 4],
    pub damage: [u8; 4],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct BrakeData {
    pub damage: [u8; 4]
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WingDamage {
    pub front_left: u8,
    pub front_right: u8,
    pub rear: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct EngineWear {
    pub mguh: u8,
    pub es: u8,
    pub ce: u8,
    pub ice: u8,
    pub mguk: u8,
    pub tc: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,
    pub car_damage_data: [CarDamageData; 22],
}
