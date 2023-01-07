use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: i8,
}

#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
pub struct WeatherForecastSample {
    pub session_type: u8,
    pub time_offset: u8,
    pub weather: u8,

    pub track_temperature: i8,
    pub track_temperature_change: i8,
    pub air_temperature: i8,
    pub air_temperature_change: i8,

    pub rain_percentage: u8,
}

#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
pub struct WeatherForecastSampleStruct {
    first_32: [WeatherForecastSample; 32],
    last: [WeatherForecastSample; 24],
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketSessionData {
    pub header: PacketHeader, // Header

    pub weather: u8, // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8, // Track temp. in degrees celsius
    pub air_temperature: i8,   // Air temp. in degrees celsius
    pub total_laps: u8,        // Total number of laps in this race
    pub track_length: u16,     // Track length in metres
    pub session_type: u8,      // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub track_id: i8, // -1 for unknown, 0-21 for tracks, see appendix
    pub formula: u8,  // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic
    pub session_time_left: u16,     // Time left in session in seconds
    pub session_duration: u16,      // Session duration in seconds
    pub pit_speed_limit: u8,        // Pit speed limit in kilometres per hour
    pub game_paused: u8,            // Whether the game is paused
    pub is_spectating: u8,          // Whether the player is spectating
    pub spectator_car_index: u8,    // Index of the car being spectated
    pub sli_pro_native_support: u8, // SLI Pro support, 0 = inactive, 1 = active
    pub num_marshall_zones: u8,     // Number of marshal zones to follow
    pub marshall_zones: [MarshalZone; 21], // List of marshal zones – max 21
    pub safety_car_status: u8,      // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub network_game: u8,                 // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8, // Number of weather samples to follow

    // #[serde(with = "BigArray")]
    pub weather_forecast_samples: WeatherForecastSampleStruct, // Array of weather forecast samples

    pub forecast_accuracy: u8,          // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,              // AI Difficulty rating – 0-110
    pub season_link_identifier: u32,    // Identifier for season - persists across saves
    pub weekend_link_identifier: u32,   // Identifier for weekend - persists across saves
    pub session_link_identifier: u32,   // Identifier for session - persists across saves
    pub pit_stop_window_ideal_lap: u8,  // Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8, // Latest lap to pit on for current strategy (player)
    pub pit_stop_rejoin_position: u8,   // Predicted position to rejoin at (player)
    pub steering_assist: u8,            // 0 = off, 1 = on
    pub braking_assist: u8,             // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: u8,             // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: u8,                 // 0 = off, 1 = on
    pub pit_release_assist: u8,         // 0 = off, 1 = on
    pub ers_assist: u8,                 // 0 = off, 1 = on
    pub drs_assist: u8,                 // 0 = off, 1 = on
    pub dynamic_racing_line: u8,        // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: u8,   // 0 = 2D, 1 = 3D
}
