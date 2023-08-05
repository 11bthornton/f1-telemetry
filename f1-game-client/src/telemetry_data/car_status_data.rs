use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct CarStatusData {
    pub traction_control: TractionControl, // Traction control - 0 = off, 1 = medium, 2 = full
    pub anti_lock_brakes: AntiLockBrakes,  // 0 (off) - 1 (on)
    pub fuel_mix: FuelMix,                 // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub front_brake_bias: u8,              // Front brake bias (percentage)
    pub pit_limiter_status: PitLimiterStatus, // Pit limiter status - 0 = off, 1 = on
    pub fuel_in_tank: f32,                 // Current fuel mass
    pub fuel_capacity: f32,                // Fuel capacity
    pub fuel_remaining_laps: f32,          // Fuel remaining in terms of laps (value on MFD)
    pub max_rpm: u16,                      // Cars max RPM, point of rev limiter
    pub idle_rpm: u16,                     // Cars idle RPM
    pub max_gears: u8,                     // Maximum number of gears
    pub drs_allowed: bool,                 // 0 = not allowed, 1 = allowed
    pub drs_activation_distance: u16, // 0 = DRS not available, non-zero - DRS will be available
    // in [X] metres
    pub actual_tyre_compound: ActualTyreCompound, // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub visual_tyre_compound: VisualTyreCompound, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub tyres_age_laps: u8,    // Age in laps of the current set of tyres
    pub vehicle_fia_flags: i8, // -1 = invalid/unknown, 0 = none, 1 = green

    pub engine_power_ice: f32,
    pub enginer_power_mguk: f32,

    // 2 = blue, 3 = yellow, 4 = red
    pub ers_store_energy: f32,       // ERS energy store in Joules
    pub ers_deploy_mode: DeployMode, // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    pub ers_harvested_this_lap_mguk: f32, // ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mguh: f32, // ERS energy harvested this lap by MGU-H
    pub ers_deployed_this_lap: f32,       // ERS energy deployed this lap
    pub network_paused: bool,             // Whether the car is paused in a network game
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(i8)]
pub enum VehicleFiaFlags {
    InvalidUnknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum VisualTyreCompound {
    None = 0,
    Soft = 16,
    Medium = 17,
    Hard = 18,
    Inter = 7,
    Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F219Wet = 15,
    F219SuperSoft = 19,
    F2Soft = 20,
    F2Medium = 21,
    F2Hard = 22,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum ActualTyreCompound {
    None = 0, // Presumably for not a player. The empty slots
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
    C0 = 21,
    Inter = 7,
    Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F215 = 15,
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketCarStatusData {
    pub header: PacketHeader,
    pub car_status_data: [CarStatusData; 22],
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum DeployMode {
    None,
    Medium,
    Hotlap,
    Overtake,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum AntiLockBrakes {
    Off,
    On,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum FuelMix {
    Lean,
    Standard,
    Rich,
    Max,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum TractionControl {
    Off,
    Medium,
    Full,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum PitLimiterStatus {
    Off,
    On,
}
