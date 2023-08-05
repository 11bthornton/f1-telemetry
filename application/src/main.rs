#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
use std::{ops::DerefMut, sync::Arc};

use app::TemplateApp;
use itertools::Itertools;

use std::sync::RwLock;
use f1_game_client::{event_loop::DataHandler, telemetry_data::{PacketCarDamageData, car_telemetry_data::PacketCarTelemetryData, PacketMotionData, PacketParticipantData, motion_data::CarMotionData, participant_data::{ParticipantData, self, Driver}, PacketSessionHistoryData, lap_data::PacketLapData, LapData}};

#[derive(Default)]
pub struct Game {
    pub damage_data: RwLock<Option<PacketCarDamageData>>,
    pub telemetry_data: RwLock<Option<PacketCarTelemetryData>>,
    pub motion_data: RwLock<Option<PacketMotionData>>,
    pub participant_data: RwLock<Option<PacketParticipantData>>,
    pub session_history_data: RwLock<Option<PacketSessionHistoryData>>,
    pub lap_data: RwLock<Option<PacketLapData>>,
    pub player_access: RwLock<std::collections::HashMap<usize, PlayerData>>

}

#[derive(Default, Clone, Copy)]
pub struct PlayerData {
    participant_data: Option<ParticipantData>,
    lap_data: Option<LapData>
}


impl DataHandler for Game {
    fn on_car_damage_data(&self, damage_data: PacketCarDamageData) {
        *self.damage_data.write().unwrap() = Some(damage_data)
    }

    fn on_car_telemetry_data(
            &self,
            telemetry_data: f1_game_client::telemetry_data::car_telemetry_data::PacketCarTelemetryData,
        ) {
        *self.telemetry_data.write().unwrap() = Some(telemetry_data)
    }

    fn on_motion_data(&self, motion_data: PacketMotionData) {
        *self.motion_data.write().unwrap() = Some(motion_data)
    }

    fn on_participant_data(&self, participant_data: PacketParticipantData) {
        *self.participant_data.write().unwrap() = Some(participant_data);

        let mapping = &mut *self.player_access.write().unwrap();

        for (index, data) in participant_data.participants.iter().enumerate() {
            
            
            let player = mapping.entry(index).or_default();
            player.participant_data = Some(*data);
        }
    }

    fn on_session_history_data(&self, session_history_data: PacketSessionHistoryData) {
        *self.session_history_data.write().unwrap() = Some(session_history_data)
    }

    fn on_lap_data(& self, lap_data: f1_game_client::telemetry_data::lap_data::PacketLapData) {
        *self.lap_data.write().unwrap() = Some(lap_data);
        let mapping = &mut *self.player_access.write().unwrap();

        for (index, data) in lap_data.lap_data.iter().enumerate() {
            
            
            let player = mapping.entry(index).or_default();
            player.lap_data = Some(*data);
        }
    }

    fn on_session_data(& self, session_data: f1_game_client::telemetry_data::PacketSessionData) {
        println!("{:#?}", session_data.weather_forecast_samples[1]);
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GAME: Game = Game::default();
}

impl Game {

    pub fn position_data(&self) -> Option<Vec<(CarMotionData, ParticipantData)>> {
        let motion_data = (*self.motion_data.read().unwrap())?.car_motion_data;
        let participant_data = (*self.participant_data.read().unwrap())?;

        let motions = motion_data.iter().take(participant_data.num_active_cars as usize).map(|x| *x);
        // let participants = participant_data.iter().map(|x| *x).filter(|x| !x.participant_id == 255);
        let participants = participant_data.participants.iter().map(|x| *x);


        Some(
            motions.zip(participants).collect_vec()
        )
    }

    pub fn pecking_order_indices(&self) -> Option<Vec<usize>> {

        let lap_data = *self.lap_data.read().unwrap();
        let participant_data = *self.participant_data.read().unwrap();

        Some(
            lap_data?
                .lap_data
                .iter()
                .enumerate()
                .take(participant_data?.num_active_cars as usize)
                .sorted_by(|(_, lap_data_1), (_, lap_data_2)| lap_data_1.car_position.cmp(&lap_data_2.car_position))
                .map(|(index, _)| index)
                .collect_vec()
        )

    }

    pub fn get_player_data(&self, index: usize) -> PlayerData {
        *self.player_access.read().unwrap().get(&index).expect("Hey")
    }

}

fn main() {
    let mut app = TemplateApp::default();

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,

        initial_window_size: Some([1280.0, 1024.0].into()),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    std::thread::spawn(|| {
        GAME.listen("192.168.1.180", "33333");
    });

    
    eframe::run_native("F1 22 Telemetry App", options, Box::new(|cc| {

        Box::new(app)
    }));
}
