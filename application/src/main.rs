#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use f1_game_client::{
    telemetry_data::{
        F1Data,
        participant_data::*,
        car_status_data::*,
        motion_data::*,
        lap_data::*,
        car_telemetry_data::*,
        session_history::*,
    },
    event_loop::{
        event_loop_generator,
        GeneratorIteratorAdapter,
    },
    
};



use std::{
    sync::{Arc, Mutex},
    thread,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref PARTICIPANTS: Arc<Mutex<Option<PacketParticipantData>>> = Arc::new(Mutex::new(None));
    static ref CAR_STATUS: Arc<Mutex<Option<PacketCarStatusData>>> = Arc::new(Mutex::new(None));
    static ref MOTION_DATA: Arc<Mutex<Option<PacketMotionData>>> = Arc::new(Mutex::new(None));
    static ref LAP_DATA: Arc<Mutex<Option<PacketLapData>>> = Arc::new(Mutex::new(None));
    static ref POSITIONS: Arc<Mutex<[u8; 22]>> = Arc::new(Mutex::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
    static ref TELEM: Arc<Mutex<Option<PacketCarTelemetryData>>> = Arc::new(Mutex::new(None));
    static ref HISTORY: Arc<Mutex<Option<PacketSessionHistoryData>>> = Arc::new(Mutex::new(None));
}

use app::TemplateApp;

#[tokio::main]
async fn main() {
    let mut app = TemplateApp::default();
    // let telemetry_data = app.state.demo.demo_windows.gallery.data.clone();


    let telemetry_data = app.state.demo.demo_windows.demos.demos[12]
        .get_thing()
        .unwrap();

    let mut iterator = GeneratorIteratorAdapter::new(event_loop_generator("20777").await);
    thread::spawn(move || {
        while let Some(f1_data) = iterator.next() {
            match f1_data {
                F1Data::Telemetry(packet_telemetry_data) => {
                    *TELEM.lock().unwrap() = Some(packet_telemetry_data)
                }
                F1Data::Participant(packet_participant) => {
                    *PARTICIPANTS.lock().unwrap() = Some(packet_participant);
                }
                F1Data::Motion(packet_motion) => {
                    *MOTION_DATA.lock().unwrap() = Some(packet_motion)
                }
                F1Data::Lap(packet_lap) => {

                    // Every time this packet comes in,
                    // this will tell us the order of the
                    // participants (ordering depends on type of event
                    // e.g. fastest lap or P1...),
                    // so we need to sort the packet by these positions
                    
                    POSITIONS.lock().unwrap().sort_by(|one, two| {
                        packet_lap
                            .lap_data[*one as usize]
                            .car_position
                            .partial_cmp(&packet_lap.lap_data[*two as usize].car_position)
                            .unwrap()
                    });
                
                    *LAP_DATA.lock().unwrap() = Some(packet_lap)
                }
                F1Data::SessionHistory(packet_history) => {
                    *HISTORY.lock().unwrap() = Some(packet_history)
                }
                
                _ => {}
            }
        }
    });

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,

        initial_window_size: Some([1280.0, 1024.0].into()),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    eframe::run_native("F1 22 Telemetry App", options, Box::new(|cc| Box::new(app)));
}
