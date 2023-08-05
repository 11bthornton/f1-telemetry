use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: VehicleFiaFlags,
}

#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8, // Time in Minutes Forecast is for
    pub weather: Weather,

    pub track_temperature: i8,
    pub track_temperature_change: i8,
    pub air_temperature: i8,
    pub air_temperature_change: i8,

    pub rain_percentage: u8,
}

// #[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
// pub struct WeatherForecastSampleStruct {
//     first_32: [WeatherForecastSample; 32],
//     last: [WeatherForecastSample; 24],
// }

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum SessionType {
    #[default]
    Unknown,
    PracticeOne,
    PracticeTwo,
    PracticeThree,
    ShortPractice,
    QualifyingOne,
    QualifyingTwo,
    QualifyingThree,
    ShortQualifying,
    OneShotQualifying,
    Race,
    RaceTwo,
    RaceThree,
    TimeTrial,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Formula {
    F1Modern,
    F1Classic,
    FormulaTwo,
    F1Generic,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum SafetyCarStatus {
    NoSafetyCar,
    FullSafetyCar,
    VirtualSafetyCar,
    FormationLap,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NetworkGame {
    Offline,
    Online,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ForecastAccuracy {
    Approximate,
    Perfect,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AssistToggle {
    Off,
    On,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum GearboxAssist {
    Manual,
    SuggestedGear,
    Automatic,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Weather {
    #[default]
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

use serde_big_array::BigArray;

use super::car_status_data::VehicleFiaFlags;

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionData {
    pub header: PacketHeader, // Header

    pub weather: Weather, // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8,     // Track temp. in degrees celsius
    pub air_temperature: i8,       // Air temp. in degrees celsius
    pub total_laps: u8,            // Total number of laps in this race
    pub track_length: u16,         // Track length in metres
    pub session_type: SessionType, // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub track: Track,     // -1 for unknown, 0-21 for tracks, see appendix
    pub formula: Formula, // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic
    pub session_time_left: u16,       // Time left in session in seconds
    pub session_duration: u16,        // Session duration in seconds
    pub pit_speed_limit: u8,          // Pit speed limit in kilometres per hour
    pub game_paused: bool,            // Whether the game is paused
    pub is_spectating: bool,          // Whether the player is spectating
    pub spectator_car_index: u8,      // Index of the car being spectated
    pub sli_pro_native_support: bool, // SLI Pro support, 0 = inactive, 1 = active
    pub num_marshall_zones: u8,       // Number of marshal zones to follow
    pub marshall_zones: [MarshalZone; 21], // List of marshal zones – max 21
    pub safety_car_status: SafetyCarStatus, // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub network_game: NetworkGame,        // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8, // Number of weather samples to follow

    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; 56], // Array of weather forecast samples

    pub forecast_accuracy: ForecastAccuracy, // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,                   // AI Difficulty rating – 0-110
    pub season_link_identifier: u32,         // Identifier for season - persists across saves
    pub weekend_link_identifier: u32,        // Identifier for weekend - persists across saves
    pub session_link_identifier: u32,        // Identifier for session - persists across saves
    pub pit_stop_window_ideal_lap: u8,       // Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,      // Latest lap to pit on for current strategy (player)
    pub pit_stop_rejoin_position: u8,        // Predicted position to rejoin at (player)
    pub steering_assist: AssistToggle,       // 0 = off, 1 = on
    pub braking_assist: AssistToggle,        // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: GearboxAssist,       // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: AssistToggle,            // 0 = off, 1 = on
    pub pit_release_assist: AssistToggle,    // 0 = off, 1 = on
    pub ers_assist: AssistToggle,            // 0 = off, 1 = on
    pub drs_assist: AssistToggle,            // 0 = off, 1 = on
    pub dynamic_racing_line: DynamicRacingLine, // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: DynamicRacingLineType, // 0 = 2D, 1 = 3D

    // added stuff
    pub speed_units_lead_player: u8,
    pub temp_units_lead_player: u8,
    pub speed_units_secondary_player: u8,
    pub temp_units_secondary_player: u8,

    pub num_safety_car_periods: u8,
    pub num_vsc_periods: u8,
    pub num_red_flag_periods: u8,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLine {
    TwoD,
    ThreeD,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLineType {
    Off,
    CornersOnly,
    Full,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(i8)]
pub enum Track {
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    SakhirBahrain = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    BakuAzerbaijan = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
    Hanoi = 25,
    Zandvoort = 26,
    Imola = 27,
    Portimao = 28,
    Jeddah = 29,
    Miami = 30,
    LasVegas = 31,
    Losail = 32,
}
