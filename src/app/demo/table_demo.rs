use std::{sync::{Arc, Mutex}, cmp::max};

use egui::{RichText, Color32};
use std::time::Duration;
use crate::telemetry_data::participant_data;

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum DemoType {
    Manual,
    ManyHomogenous,
    ManyHeterogenous,
}

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableDemo {
    demo: DemoType,
    resizable: bool,
    num_rows: usize,

}

impl Default for TableDemo {
    fn default() -> Self {
        Self {
            demo: DemoType::Manual,
            resizable: true,
            num_rows: 10_000,
        }
    }
}

impl super::Demo for TableDemo {
    fn get_thing(
        &mut self,
    ) -> Option<Arc<Mutex<crate::telemetry_data::car_telemetry_data::PacketCarTelemetryData>>> {
        None
    }
    fn name(&self) -> &'static str {
        "☰ Live Classification"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for TableDemo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        

        // Leave room for the source code link after the table demo:
        use egui_extras::{Size, StripBuilder};
        StripBuilder::new(ui)
            .size(Size::remainder()) // for the table
            .size(Size::exact(10.0)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    self.table_ui(ui);
                });
                strip.cell(|ui| {
                    ui.vertical_centered(|ui| {
                        // ui.add(crate::egui_github_link_file!());
                    });
                });
            });
    }
}

impl TableDemo {
    fn table_ui(&mut self, ui: &mut egui::Ui) {

        let lap_data = crate::LAP_DATA.lock().unwrap();
        let participant_data = crate::PARTICIPANTS.lock().unwrap();

        if lap_data.is_none() {
            return;
        }

        use egui_extras::{Size, TableBuilder};

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Size::initial(80.0).at_least(40.0))
            .column(Size::initial(90.0).at_least(40.0))
            .column(Size::initial(180.0).at_least(40.0))
            .column(Size::initial(150.0).at_least(40.0))
            .column(Size::initial(150.0).at_least(40.0))
            .column(Size::initial(150.0).at_least(40.0))
            .column(Size::remainder().at_least(150.0))
            .resizable(self.resizable)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Position");
                });
                header.col(|ui| {
                    ui.heading("Driver");
                });
                header.col(|ui| {
                    ui.heading("Last Lap Time");
                });
                header.col(|ui| {
                    
                    ui.heading("Current Lap Time");
                });
                header.col(|ui| {
                    ui.heading("Sector 1 Time");
                });
                header.col(|ui| {
                    ui.heading("Sector 2 Time");
                });
                header.col(|ui| {
                    ui.heading("Sector 3 Time");
                });
            })
            .body(|mut body| match self.demo {
                

                DemoType::Manual => {
                    let  lap_data = &lap_data.as_ref().unwrap().lap_data;
                    let participant_data = participant_data.as_ref().unwrap();

                    for (index, position) in crate::POSITIONS.lock().unwrap().iter().enumerate() {
                        let row_height = 18.0;
                        let participant = &participant_data.participants[*position as usize];
                        let participant_lap_data = &lap_data[*position as usize];

                        if participant.driver_id == 255 {
                            continue;
                        }

                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label((index - 1).to_string());
                            });
                            row.col(|ui| {
                                
                                ui.label(RichText::new(participant.name()).color(Color32::from_rgb(200, 50, 200)));

                            });
                            row.col(|ui| {
                                let duration = Duration::from_millis(participant_lap_data.last_lap_time_in_ms.into());

                                ui.label(format!("{:?}", duration));
                            });
                            row.col(|ui| {
                                let duration = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());
                                
                                ui.label(format!("{:?}", duration));
                            });
                            row.col(|ui| {
                                let duration = Duration::from_millis(participant_lap_data.sector_1_time_in_ms.into());
                                
                                ui.label(format!("{:?}", duration));
                            });
                            row.col(|ui| {
                                
                                let duration_1 = Duration::from_millis(participant_lap_data.sector_1_time_in_ms.into());
                                let duration_lap = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());

                                let acc = max(duration_lap - duration_1, Duration::from_millis(0));
                                
                                ui.label(format!("{:?}", acc));
                            });
                            row.col(|ui| {
                                let duration_2 = Duration::from_millis(participant_lap_data.sector_2_time_in_ms.into());
                                let duration_lap = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());

                                let acc = max(duration_lap - duration_2, Duration::from_millis(0));
                                
                                ui.label(format!("{:?}", acc));
                            });
                        });
                    }

          
                }
                DemoType::ManyHomogenous => {
                    body.rows(text_height, self.num_rows, |row_index, mut row| {
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(clock_emoji(row_index));
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new("Thousands of rows of even height").wrap(false),
                            );
                        });
                    });
                }
                DemoType::ManyHeterogenous => {
                    fn row_thickness(row_index: usize) -> f32 {
                        if thick_row(row_index) {
                            30.0
                        } else {
                            18.0
                        }
                    }
                    body.heterogeneous_rows(
                        (0..self.num_rows).into_iter().map(row_thickness),
                        |row_index, mut row| {
                            row.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label(row_index.to_string());
                                });
                            });
                            row.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label(clock_emoji(row_index));
                                });
                            });
                            row.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.style_mut().wrap = Some(false);
                                    if thick_row(row_index) {
                                        ui.heading("Extra thick row");
                                    } else {
                                        ui.label("Normal row");
                                    }
                                });
                            });
                        },
                    );
                }
            });
    }
}

fn clock_emoji(row_index: usize) -> String {
    char::from_u32(0x1f550 + row_index as u32 % 24)
        .unwrap()
        .to_string()
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}
