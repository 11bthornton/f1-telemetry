use std::{
    sync::{Arc, Mutex},
    thread,
};
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};

#[allow(dead_code)]
struct Telemetry {
    car_motion_data: Arc<Mutex<PacketMotionData>>,
    texture: Option<egui::TextureHandle>,
    lap_data: Arc<Mutex<PacketLapData>>,
    track_drawer: Vec<(f32, f32)>,
    x: f32,
    z: f32,
    scale: f32,
}

use image;
use crate::telemetry_data::lap_data::PacketLapData;
use crate::telemetry_data::motion_data::PacketMotionData;
use crate::telemetry_data::F1Data;
use crate::event_loop::{event_loop_generator, GeneratorIteratorAdapter};

async fn ui() {
    let options = eframe::NativeOptions::default();
    let app = Telemetry::default();

    let car_motion_data = app.car_motion_data.clone();
    let car_lap_data = app.lap_data.clone();

    let mut iterator = GeneratorIteratorAdapter::new(event_loop_generator("20777").await);

    thread::spawn(move || {
        while let Some(f1_data) = iterator.next() {
            match f1_data {
                F1Data::Motion(packet_car_motion_data) => {
                    *car_motion_data.lock().unwrap() = packet_car_motion_data;
                }
                F1Data::Lap(packet_lap_data) => *car_lap_data.lock().unwrap() = packet_lap_data,
                _ => {}
            }
        }
    });

    eframe::run_native("Test", options, Box::new(move |_cc| Box::new(app)));
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            car_motion_data: Arc::new(Mutex::new(PacketMotionData::default())),
            texture: None,
            lap_data: Arc::new(Mutex::new(PacketLapData::default())),
            track_drawer: vec![],
            x: 782.0,
            z: 412.0,
            scale: 1.22,
        }
    }
}

impl eframe::App for Telemetry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                // Load the texture only once.
                ui.ctx().load_texture(
                    "imola",
                    load_image_from_path("./resources/maps/imola.png").unwrap(),
                    egui::TextureFilter::Linear,
                )
            });
            ui.image(texture, texture.size_vec2());

            let car_motions = &*self.car_motion_data.lock().unwrap();
            let car_laps = &*self.lap_data.lock().unwrap();
            ui.heading(format!(
                "HERE {} ms",
                car_laps.lap_data[0].current_lap_time_in_ms.to_string()
            ));

            
        });
        ctx.request_repaint();
    }
}

fn load_image_from_path(path_name: &str) -> Result<egui::ColorImage, image::ImageError> {
    let path = std::path::Path::new(path_name);
    let mut image = image::io::Reader::open(path)?.decode()?;
    image = image.rotate180();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
