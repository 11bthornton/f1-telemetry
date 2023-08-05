use crate::telemetry_data::packet_header::PacketHeader;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Debug, Default, Serialize, Clone, Copy)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,        // Last lap time in milliseconds
    pub current_lap_time_in_ms: u32,     // Current time around the lap in milliseconds
    pub sector1_time_in_ms: u16,         // Sector 1 time in milliseconds
    pub sector1_time_minutes: u8,        // Sector 1 whole minute part
    pub sector2_time_in_ms: u16,         // Sector 2 time in milliseconds
    pub sector2_time_minutes: u8,        // Sector 2 whole minute part
    delta_to_car_in_front_in_ms: u16,    // Time delta to car in front in milliseconds
    delta_to_race_leader_in_ms: u16,     // Time delta to race leader in milliseconds
    lap_distance: f32, // Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    total_distance: f32, // Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    safety_car_delta: f32, // Delta in seconds for safety car
    pub car_position: u8, // Car race position
    current_lap_num: u8, // Current lap number
    pit_status: PitStatus, // 0 = none, 1 = pitting, 2 = in pit area
    num_pit_stops: u8,   // Number of pit stops taken in this race
    sector: Sector,      // 0 = sector1, 1 = sector2, 2 = sector3
    current_lap_invalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    penalties: u8,       // Accumulated time penalties in seconds to be added
    total_warnings: u8,  // Accumulated number of warnings issued
    corner_cutting_warnings: u8, // Accumulated number of corner cutting warnings issued
    num_unserved_drive_through_pens: u8, // Num drive through pens left to serve
    num_unserved_stop_go_pens: u8, // Num stop go pens left to serve
    grid_position: u8,   // Grid position the vehicle started the race in
    driver_status: DriverStatus, // Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    result_status: ResultStatus, // Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = didnotfinish, 5 = disqualified, 6 = not classified, 7 = retired
    pit_lane_timer_active: bool, // Pit lane timing, 0 = inactive, 1 = active
    pit_lane_time_in_lane_in_ms: u16, // If active, the current time spent in the pit lane in ms
    pit_stop_timer_in_ms: u16,   // Time of the actual pit stop in ms
    pit_stop_should_serve_pen: u8, // Whether the car should serve a penalty at this stop
}

#[derive(Deserialize, Debug, Default, Serialize, Clone, Copy)]
pub struct PacketLapData {
    pub header: PacketHeader,    // Header
    pub lap_data: [LapData; 22], // Lap data for all cars on track

    pub time_trial_pb_car_idx: u8,
    pub m_time_trial_rival_car_idx: u8,
}

#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum Sector {
    #[default]
    SectorOne = 0,
    SectorTwo = 1,
    SectorThree = 2,
}

#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum PitStatus {
    #[default]
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum DriverStatus {
    InGarage = 0,
    #[default]
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum ResultStatus {
    #[default]
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    DNF = 4,
    DSQ = 5,
    NotClassified = 6,
    Retired = 7,
}
