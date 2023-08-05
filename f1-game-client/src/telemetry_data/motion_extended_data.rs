use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PacketMotionExData {
    header: PacketHeader, // Header
    // Extra player car ONLY data
    suspension_position: [f32; 4], // Note: All wheel arrays have the following order:
    suspension_velocity: [f32; 4], // RL, RR, FL, FR
    suspension_acceleration: [f32; 4], // RL, RR, FL, FR
    wheel_speed: [f32; 4],         // Speed of each wheel
    wheel_slip_ratio: [f32; 4],    // Slip ratio for each wheel
    wheel_slip_angle: [f32; 4],    // Slip angles for each wheel
    wheel_lat_force: [f32; 4],     // Lateral forces for each wheel
    wheel_long_force: [f32; 4],    // Longitudinal forces for each wheel
    height_of_cog_above_ground: f32, // Height of centre of gravity above ground
    local_velocity_x: f32,         // Velocity in local space – metres/s
    local_velocity_y: f32,         // Velocity in local space
    local_velocity_z: f32,         // Velocity in local space
    angular_velocity_x: f32,       // Angular velocity x-component – radians/s
    angular_velocity_y: f32,       // Angular velocity y-component
    angular_velocity_z: f32,       // Angular velocity z-component
    angular_acceleration_x: f32,   // Angular acceleration x-component – radians/s/s
    angular_acceleration_y: f32,   // Angular acceleration y-component
    angular_acceleration_z: f32,   // Angular acceleration z-component
    front_wheels_angle: f32,       // Current front wheels angle in radians
    wheel_vert_force: [f32; 4],    // Vertical forces for each wheel
}
