use crate::telemetry_data::packet_header::PacketHeader;
use serde::{Deserialize, Serialize};
use std::str;

#[allow(unused_attributes)]
#[macro_use]
use serde_big_array::BigArray;

#[derive(Deserialize, Debug, Serialize)]
pub struct ParticipantData {

    pub ai_controlled: u8,
    pub driver_id: u8,
    pub network_id: u8,
    pub team_id: u8,
    pub my_team: u8,
    pub race_number: u8,
    pub nationality: u8,

    #[serde(with = "BigArray")]
    pub name: [u8; 48],

    pub telemetry: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PacketParticipantData {
    pub header: PacketHeader,
    pub num_active_cars: u8,
    pub participants: [ParticipantData; 22],
}

impl ParticipantData {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        let name = str::from_utf8(&self.name).unwrap();

        format!("{}", name)
    }

    pub fn team_colour(&self) -> (u8, u8, u8) {
        match self.team_id {
            
            0 => (0, 210, 90),
            1 => (220, 0, 0),
            2 => (6, 0, 239),
            3 => (0, 90, 255),
            4 => (0, 111, 98),
            5 => (0, 144, 255),
            6 => (43, 69, 98),
            7 => (255, 255, 255),
            8 => (255, 235, 0),
            9 => (144, 0, 0),
            _ => (0, 0 ,0)
        }
    }
}
