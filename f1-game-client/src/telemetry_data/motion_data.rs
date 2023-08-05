use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, Copy, Clone)]
pub struct CarMotionData {
    pub world_position_x: f32,     // World space X position
    pub world_position_y: f32,     // World space Y position
    pub world_position_z: f32,     // World space Z position
    pub world_velocity_x: f32,     // Velocity in world space X
    pub world_velocity_y: f32,     // Velocity in world space Y
    pub world_velocity_z: f32,     // Velocity in world space Z
    pub world_forward_dir_x: i16,  // World space forward X direction (normalised)
    pub world_forward_dir_y: i16,  // World space forward Y direction (normalised)
    pub world_forward_dir_z: i16,  // World space forward Z direction (normalised)
    pub world_right_dir_x: i16,    // World space right X direction (normalised)
    pub world_right_dir_y: i16,    // World space right Y direction (normalised)
    pub world_right_dir_z: i16,    // World space right Z direction (normalised)
    pub g_force_lateral: f32,      // Lateral G-Force component
    pub g_force_longitudinal: f32, // Longitudinal G-Force component
    pub g_force_vertical: f32,     // Vertical G-Force component
    pub yaw: f32,                  // Yaw angle in radians
    pub pitch: f32,                // Pitch angle in radians
    pub roll: f32,                 // Roll angle in radians
}

#[derive(Deserialize, Debug, Default, Serialize, Clone, Copy)]
pub struct PacketMotionData {
    pub header: PacketHeader, // Header

    pub car_motion_data: [CarMotionData; 22], // Data for all cars on track

                                              // // Extra player car ONLY data
                                              // pub suspension_position: [f32; 4], // Note: All wheel arrays have the following order:
                                              // pub suspension_velocity: [f32; 4], // RL, RR, FL, FR
                                              // pub suspension_acceleration: [f32; 4], // RL, RR, FL, FR
                                              // pub wheel_speed: [f32; 4],         // Speed of each wheel
                                              // pub wheel_slip: [f32; 4],          // Slip ratio for each wheel

                                              // pub local_velocity_x: f32,       // Velocity in local space
                                              // pub local_velocity_y: f32,       // Velocity in local space
                                              // pub local_velocity_z: f32,       // Velocity in local space
                                              // pub angular_velocity_x: f32,     // Angular velocity x-component
                                              // pub angular_velocity_y: f32,     // Angular velocity y-component
                                              // pub angular_velocity_z: f32,     // Angular velocity z-component
                                              // pub angular_acceleration_x: f32, // Angular velocity x-component
                                              // pub angular_acceleration_y: f32, // Angular velocity y-component
                                              // pub angular_acceleration_z: f32, // Angular velocity z-component
                                              // pub front_wheels_angle: f32,     // Current front wheels angle in radians
}
