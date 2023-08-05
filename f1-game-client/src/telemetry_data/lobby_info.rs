use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};
use std::str;

#[allow(unused_attributes)]
#[macro_use]
use serde_big_array::BigArray;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::telemetry_data::participant_data::{Nationality, Team};

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct LobbyInfoData {
    pub ai_controlled: bool, // Whether the vehicle is AI (1) or Human (0) controlled
    pub team: Team,          // Team id - see appendix (255 if no team currently selected)
    pub nationality: Nationality, // Nationality of the driver
    pub platform: Platform,  // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown

    #[serde(with = "BigArray")]
    pub name: [u8; 48], // Name of participant in UTF-8 format – null terminated. Will be truncated with ... (U+2026) if too long
    pub car_number: u8,            // Car number of the player
    pub ready_status: ReadyStatus, // 0 = not ready, 1 = ready, 2 = spectating
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketLobbyInfoData {
    header: PacketHeader, // Header
    // Packet specific data
    num_players: u8, // Number of players in the lobby data
    lobby_players: [LobbyInfoData; 22],
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum Platform {
    SuperUnknown = 0,
    Steam = 1,
    Playstation = 3,
    Xbox = 4,
    Origin = 6,
    Unknown = 255,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum ReadyStatus {
    NotReady,
    Ready,
    Spectating,
}
