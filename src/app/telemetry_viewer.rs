use egui::Pos2;
use egui::Color32;
use egui::Stroke;
use egui::color;
use egui::plot::Legend;
use egui::plot::Line;
use egui::plot::LineStyle;
use egui::plot::MarkerShape;
use egui::plot::Plot;
use egui::plot::PlotPoint;
use egui::plot::PlotPoints;
use egui::plot::Points;
use egui::plot::Text;

use crate::PARTICIPANTS;
use crate::{telemetry_data::car_telemetry_data::PacketCarTelemetryData, MOTION_DATA};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref OUTERS : Vec<(f32, f32)> = {

        let mut file = File::open("austria.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str::<Vec<(f32, f32)>>(&contents).unwrap().iter().skip(1).step_by(2).copied().collect()
            
    };
}


#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TelemetryViewerApp {
    pub data: Arc<Mutex<PacketCarTelemetryData>>,
    x: f32,
    z: f32,
    scale: f32,
    rotation: f32,
    track_drawer: Vec<(f32, f32)>,
}

impl eframe::App for TelemetryViewerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

impl Default for TelemetryViewerApp {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(PacketCarTelemetryData::default())),
            // x: 880.0,
            // z: 460.0,
            // scale: 1.00,
            // rotation: 260.0,
            x: 930.0,
            z: 370.0,
            scale: 0.8,
            rotation: 150.0,
            track_drawer: Default::default()
        }
    }
}

impl TelemetryViewerApp {

    fn track_map(&self) -> Line {

        Line::new(
            PlotPoints::new(
                OUTERS.iter().map(|(x, y)| [*x as f64,  - *y as f64]).collect()
            )
        )
        .color(Color32::from_rgb(255, 100, 100))
        .style(LineStyle::Solid)

    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let mut plot = Plot::new("live map").legend(Legend::default());
            plot = plot.show_axes([false, false]);
            plot = plot.view_aspect(1.0);
            plot = plot.data_aspect(1.0);
            
            if ui.add(egui::Button::new("Clear")).clicked() {
                self.track_drawer.clear();
            }
            if ui.add(egui::Button::new("Save Inner")).clicked() {
                let mut f = File::create("./austria-inner.txt").expect("Unable to create file");
                f.write_all(serde_json::to_string(&self.track_drawer).unwrap().as_bytes()).expect("Unable to write data");
            }
            if ui.add(egui::Button::new("Save Outer")).clicked() {
                let mut f = File::create("./austria-outer.txt").expect("Unable to create file");
                f.write_all(serde_json::to_string(&self.track_drawer).unwrap().as_bytes()).expect("Unable to write data");
            }

            

            ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0));

            plot.show(ui, |ui| {
                ui.line(self.track_map());
                let mut index = 0;
                if let Some(car_motions) = &*MOTION_DATA.lock().unwrap() {
                    for (car, participant) in car_motions.car_motion_data.iter().zip(
                        PARTICIPANTS.lock().unwrap().as_ref().unwrap().participants.iter()
                    ) {
                        ui.points(
                            Points::new(
                                vec![[car.world_position_x as f64, -car.world_position_z as f64]]
                            )
                            .name(participant.name())
                            .radius(5.0)
                            .color(Color32::from_rgb(participant.team_colour().0,participant.team_colour().1, participant.team_colour().2))
                            .shape(MarkerShape::Circle)
                            
                      
                        );
                        if index == 0 {
                            self.track_drawer.push((car.world_position_x, car.world_position_z));
                        }
                        index += 1;
                        ui.text(Text::new(PlotPoint::new(car.world_position_x, -car.world_position_z + 10.0), participant.name()));     
                    }
                }
                
            });


            if ui.add(egui::Button::new("Save State")).clicked() {
                let string = serde_json::to_string(&self.track_drawer).unwrap();
                let mut file = File::create("./austria.txt").unwrap();
                file.write_all(string.as_bytes()).unwrap();
            }

            if ui.add(egui::Button::new("Clear")).clicked() {
                self.track_drawer.clear();
            }

        });
    }
}