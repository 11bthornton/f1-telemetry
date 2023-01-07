use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, Clone)]
pub struct PacketHeader {
    #[serde(rename(deserialize = "m_packetFormat"))]
    pub packet_format: u16,

    #[serde(rename(deserialize = "m_gameMajorVersion"))]
    pub game_major_version: u8,

    #[serde(rename(deserialize = "m_gameMinorVersion"))]
    pub game_minor_version: u8,

    #[serde(rename(deserialize = "m_packetVersion"))]
    pub packet_version: u8,

    #[serde(rename(deserialize = "m_packetId"))]
    pub packet_id: u8,

    #[serde(rename(deserialize = "m_sessionUID"))]
    pub session_uid: u64,

    #[serde(rename(deserialize = "m_sessionTime"))]
    pub session_time: f32,

    #[serde(rename(deserialize = "m_frameIdentifier"))]
    pub frame_identifier: u32,

    #[serde(rename(deserialize = "m_carPlayerIndex"))]
    pub player_car_index: u8,

    #[serde(rename(deserialize = "m_secondaryPlayerCarIndex"))]
    pub secondary_player_car_index: u8,
}
