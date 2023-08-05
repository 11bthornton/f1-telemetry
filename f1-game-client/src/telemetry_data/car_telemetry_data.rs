use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader,
    pub telemetry_data: [CarTelemetryData; 22],
    pub mfd_panel_index: u8,
    pub mfd_panel_index_secondary_player: u8,
    pub suggested_gear: i8,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: u8,
    pub rev_lights_percent: u8,
    pub rev_light_bit: u16,
    pub brake_temps: [u16; 4],
    pub tyre_surface_temps: [u8; 4],
    pub tyre_inner_temps: [u8; 4],
    pub engine_temp: u16,
    pub tyre_pressures: [f32; 4],
    pub surface_types: [u8; 4],
}
