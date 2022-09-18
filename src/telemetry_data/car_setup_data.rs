use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    pub car_setips: [CarSetupData; 22],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CarSetupData {
    pub front_wing: u8,                // Front wing aero
    pub rear_wing: u8,                 // Rear wing aero
    pub on_throttle: u8,               // Differential adjustment on throttle (percentage)
    pub off_throttle: u8,              // Differential adjustment off throttle (percentage)
    pub front_camber: f32,             // Front camber angle (suspension geometry)
    pub rear_camber: f32,              // Rear camber angle (suspension geometry)
    pub front_toe: f32,                // Front toe angle (suspension geometry)
    pub rear_toe: f32,                 // Rear toe angle (suspension geometry)
    pub front_suspension: u8,          // Front suspension
    pub rear_suspension: u8,           // Rear suspension
    pub front_anti_roll_bar: u8,       // Front anti-roll bar
    pub rear_anti_roll_bar: u8,        // Front anti-roll bar
    pub front_suspension_height: u8,   // Front ride height
    pub rear_suspension_height: u8,    // Rear ride height
    pub brake_pressure: u8,            // Brake pressure (percentage)
    pub brake_bias: u8,                // Brake bias (percentage)
    pub rear_left_tyre_pressure: u8,   // Rear left tyre pressure (PSI)
    pub rear_right_tyre_pressure: u8,  // Rear right tyre pressure (PSI)
    pub front_left_typre_pressure: u8, // Front left tyre pressure (PSI)
    pub front_right_tyre_pressure: u8, // Front right tyre pressure (PSI)
    pub ballast: u8,                   // Ballast
    pub fuel_load: u8,                 // Fuel load
}
