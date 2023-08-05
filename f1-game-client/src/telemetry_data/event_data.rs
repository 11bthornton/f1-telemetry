use std::str;

use crate::telemetry_data::packet_header::PacketHeader;
use bincode::deserialize;
use serde::{Deserialize, Serialize};
use std::str::Utf8Error;

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub remaining_data: [u8; 16],
}

macro_rules! decode {
    ($($name:expr => ($ty:ty,  $var:ident)),+) => {
            impl PacketEventData {
                // #[ignore(unreachable_code)]
                pub fn decode(self) -> Result<PacketEventFinal, Utf8Error> {

                        let button_code = str::from_utf8(&self.event_string_code)?;

                        match button_code {
                            $(
                                $name => {

                                        let decoded: $ty = deserialize(&self.remaining_data).unwrap();

                                         return Ok(
                                            PacketEventFinal {
                                                m_header: self.m_header,
                                                event_string_code: self.event_string_code,
                                                r#type: EventType::$var(decoded)
                                            }
                                        )


                                },
                            )+

                            _ => panic!(
                                "{}", "Unrecognised button code! {button_code}"
                            )
                        }

                }
            }
    };
}

decode!(

    "BUTN" => (Buttons, Buttons),
    "RCWN" => (RaceWinner, RaceWinner),
    "FLBK" => (FlashBack, FlashBack),
    "TMPT" => (TeamMateInPits, TeamMateInPits),
    "RCWM" => (RaceWinner, RaceWinner),
    "RTMT" => (Retirement, Retirement),
    "FTLP" => (FastestLap, FastestLap),
    "STLG" => (StartLights, StartLights),
    "SPTP" => (SpeedTrap, SpeedTrap),
    "PENA" => (Penalty, Penalty),
    "DTSV" => (DriveThroughPenaltyServed, DriveThroughPenaltyServed),
    "SGSV" => (StopGoPenaltyServed, StopGoPenaltyServed),
    "LGOT" => (LightsOut, LightsOut),
    "SSTA" => (SessionStart, SessionStart),
    "SEND" => (SessionEnd, SessionEnd),
    "CHQF" => (ChequeredFlag, ChequeredFlag),
    "DRSE" => (DrsEnabled, DrsEnabled),
    "DRSD" => (DrsDisabled, DrsDisabled),
    "OVTK" => (Overtake, Overtake),
    "RDFL" => (RedFlag, RedFlag)

);

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct PacketEventFinal {
    pub m_header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub r#type: EventType,
}
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct Overtake {
    pub overtaking_vehicle_idx: u8,
    pub overtaken_vehicle_idx: u8,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct RedFlag;

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Buttons {
    button_status: u32,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct FastestLap {
    vehicle_index: u8,
    lap_time: f32,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Retirement {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct TeamMateInPits {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct RaceWinner {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub vehicle_index: u8,
    pub time: u8,
    pub lap_number: u8,
    pub places_gained: u8,
}

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PenaltyType {
    DriveThrough = 0,
    StopGo = 1,
    GridPenalty = 2,
    PenaltyReminder = 3,
    TimePenalty = 4,
    Warning = 5,
    Disqualified = 6,
    RemovedFromFormationLap = 7,
    ParkedTooLongTimer = 8,
    TyreRegulations = 9,
    ThisLapInvalidated = 10,
    ThisAndNextLapInvalidated = 11,
    ThisLapInvalidatedWithoutReason = 12,
    ThisAndNextLapInvalidatedWithoutReason = 13,
    ThisAndPreviousLapInvalidated = 14,
    ThisAndPreviousLapInvalidatedWithoutReason = 15,
    Retired = 16,
    BlackFlagTimer = 17,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum InfringementType {
    BlockingBySlowDriving = 0,
    BlockingByWrongWayDriving = 1,
    ReversingOffTheStartLine = 2,
    BigCollision = 3,
    SmallCollision = 4,
    CollisionFailedToHandBackPositionSingle = 5,
    CollisionFailedToHandBackPositionMultiple = 6,
    CornerCuttingGainedTime = 7,
    CornerCuttingOvertakeSingle = 8,
    CornerCuttingOvertakeMultiple = 9,
    CrossedPitExitLane = 10,
    IgnoringBlueFlags = 11,
    IgnoringYellowFlags = 12,
    IgnoringDriveThrough = 13,
    TooManyDriveThroughs = 14,
    DriveThroughReminderServeWithinNLaps = 15,
    DriveThroughReminderServeThisLap = 16,
    PitLaneSpeeding = 17,
    ParkedForTooLong = 18,
    IgnoringTyreRegulations = 19,
    TooManyPenalties = 20,
    MultipleWarnings = 21,
    ApproachingDisqualification = 22,
    TyreRegulationsSelectSingle = 23,
    TyreRegulationsSelectMultiple = 24,
    LapInvalidatedCornerCutting = 25,
    LapInvalidatedRunningWide = 26,
    CornerCuttingRanWideGainedTimeMinor = 27,
    CornerCuttingRanWideGainedTimeSignificant = 28,
    CornerCuttingRanWideGainedTimeExtreme = 29,
    LapInvalidatedWallRiding = 30,
    LapInvalidatedFlashbackUsed = 31,
    LapInvalidatedResetToTrack = 32,
    BlockingThePitlane = 33,
    JumpStart = 34,
    SafetyCarToCarCollision = 35,
    SafetyCarIllegalOvertake = 36,
    SafetyCarExceedingAllowedPace = 37,
    VirtualSafetyCarExceedingAllowedPace = 38,
    FormationLapBelowAllowedSpeed = 39,
    FormationLapParking = 40,
    RetiredMechanicalFailure = 41,
    RetiredTerminallyDamaged = 42,
    SafetyCarFallingTooFarBack = 43,
    BlackFlagTimer = 44,
    UnservedStopGoPenalty = 45,
    UnservedDriveThroughPenalty = 46,
    EngineComponentChange = 47,
    GearboxChange = 48,
    ParcFerméChange = 49,
    LeagueGridPenalty = 50,
    RetryPenalty = 51,
    IllegalTimeGain = 52,
    MandatoryPitstop = 53,
    AttributeAssigned = 54,
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct SpeedTrap {
    pub vehicle_index: u8,
    pub speed: f32,
    pub overall_fastest_in_session: u8,
    pub driver_fastest_in_session: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct StartLights {
    pub num_lights: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct DriveThroughPenaltyServed {
    pub vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct StopGoPenaltyServed {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct FlashBack {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub enum EventType {
    Buttons(Buttons),
    FastestLap(FastestLap),
    Retirement(Retirement),
    TeamMateInPits(TeamMateInPits),
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    FlashBack(FlashBack),
    LightsOut(LightsOut),
    SessionStart(SessionStart),
    SessionEnd(SessionEnd),
    DrsEnabled(DrsEnabled),
    DrsDisabled(DrsDisabled),
    ChequeredFlag(ChequeredFlag),
    AlternateSpeedTrap(AlternateSpeedTrap),
    Overtake(Overtake),
    RedFlag(RedFlag),
    #[default]
    NoEvent,
}
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct LightsOut;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct SessionStart;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct SessionEnd;

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct DrsEnabled;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct DrsDisabled;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct ChequeredFlag;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct AlternateSpeedTrap;
