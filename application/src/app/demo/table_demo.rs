use std::{sync::{Arc, Mutex}, cmp::max, fmt::Display};

use egui::{RichText, Color32};
use std::time::Duration;
use f1_game_client::telemetry_data::{participant_data, session_history::PacketSessionHistoryData};

use crate::app::demo::toggle_switch::toggle;

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
    toggle: bool,
    data: [Option<PacketSessionHistoryData>; 22]
}

impl Default for TableDemo {
    fn default() -> Self {
        Self {
            demo: DemoType::Manual,
            resizable: true,
            num_rows: 10_000,
            toggle: true,
            data: Default::default()
        }
    }
}

impl super::Demo for TableDemo {
    fn get_thing(
        &mut self,
    ) -> Option<Arc<Mutex<f1_game_client::telemetry_data::car_telemetry_data::PacketCarTelemetryData>>> {
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

        ui.horizontal(|ui| {
            ui.label("Show Best Lap Time");
            ui.add(toggle(&mut self.toggle));
        });

        if crate::HISTORY.lock().unwrap().is_none() {
            return;
        }

        let history = crate::HISTORY.lock().unwrap();

        if history.is_none() {
            return;
        }

        let history = history.as_ref().unwrap();

        self.data[history.car_index as usize] = Some(history.clone());

        let participant_data = crate::PARTICIPANTS.lock().unwrap();
        if participant_data.is_none() {
            return;
        }

        let uid = participant_data.as_ref().unwrap().header.session_uid;
        

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
                    ui.heading(format!("Position {}", uid));
                });
                header.col(|ui| {
                    ui.heading("Driver");
                });
                header.col(|ui| {
                    ui.heading("Best Lap Time");
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
            .body(|mut body| {
                

                
                    // let  lap_data = &lap_data.as_ref().unwrap().lap_data;
                    let participant_data = participant_data.as_ref().unwrap();

                    for (index, position) in crate::POSITIONS.lock().unwrap().iter().enumerate() {
                        let row_height = 18.0;
                        let participant = &participant_data.participants[*position as usize];

                        // let participant_lap_data = &lap_data[*position as usize];
                        let history_data = &self.data[*position as usize];

                        // if history_data.is_none() {
                        //     continue;
                        // }

                        // let history_data = history_data.as_ref().unwrap();

                        if participant.driver_id == 255 {
                            continue;
                        }

                        body.row(row_height, |mut row| {

                            row.col(|ui| {
                                ui.label((index - 1).to_string());
                            });
                            row.col(|ui| {
                                
                                let name_label = if index == 2 {
                                    ui.label(
                                RichText::new(participant.name())
                                        .color(Color32::from_rgb(200, 50, 200))
                                    )
                                } else {
                                    ui.label(participant.name())
                                };

                                name_label.clone().on_hover_ui(|ui| {

                                    ui.label(history_data.as_ref().unwrap().best_lap_time_num.to_string());

                                    ui.label(
                                        format!("{:#?}", history_data.as_ref().unwrap().concat())
                                    );
                                });

                                name_label.context_menu(|ui| {
                                    ui.label("fucking epic!");
                                });

                            });
                            row.col(|ui| {
                                // let duration = Duration::from_millis(participant_lap_data.last_lap_time_in_ms.into());
                                
                                let dur = if history_data.as_ref().unwrap().best_lap_time_num == 0 {
                                    None
                                } else {
                                    let time = Duration::from_millis(
                                        history_data.as_ref().unwrap().concat()[history_data.as_ref().unwrap().best_lap_time_num as usize - 1].lap_time_in_ms.into()
                                    );
    
                                    Some(time)
                                };

                                ui.label(
                                    format!("{:?}", pretty_print_duration(dur.as_ref()))
                                );
                            });
                            row.col(|ui| {

                                // let dur = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());

                                // let duration = if dur.is_zero() {
                                //     None
                                // } else {
                                //     Some(&dur)
                                // };
                                
                                ui.label(format!("{}", "Filler"));
                            });
                            row.col(|ui| {
                                // let duration = Duration::from_millis(participant_lap_data.sector_1_time_in_ms.into());
                                
                                // if duration.is_zero() {
                                //     // ui.label(format!("{:?}", Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into())));
                                //     ui.label(format!("{:?}", "Filler"));

                                // } else {
                                //     ui.label(format!("{:?}", "Filler"));
                                // }
                                
                                ui.label(format!("{:?}", "Filler"));
                                

                            });
                            row.col(|ui| {
                                
                                // let duration_1 = Duration::from_millis(participant_lap_data.sector_1_time_in_ms.into());
                                // let duration_2 = Duration::from_millis(participant_lap_data.sector_2_time_in_ms.into());
                                // let duration_lap = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());

                                // if duration_1.is_zero() {
                                //     ui.label(format!("{:?}", Duration::from_millis(0)));
                                // } else {
                                //     if duration_2.is_zero() {
                                //         ui.label(format!("{:?}", duration_lap - duration_1));
                                //     } else {
                                //         ui.label(format!("{:?}", duration_2));
                                //     }
                                // }

                                ui.label("Filler");
                                
                            });
                            row.col(|ui| {
                                // let duration_1 = Duration::from_millis(participant_lap_data.sector_1_time_in_ms.into());
                                // let duration_2 = Duration::from_millis(participant_lap_data.sector_2_time_in_ms.into());
                                // let duration_lap = Duration::from_millis(participant_lap_data.current_lap_time_in_ms.into());

                                // if duration_2.is_zero() {
                                //     ui.label(format!("{:?} FIRST BRANCH", Duration::from_millis(0)));
                                // } else {
                                //     ui.label(format!("{:?}", duration_lap));
                                // }
                                
                                ui.label("Filler");
                                
                            });
                        });
                    }
                }

          
                );
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

fn pretty_print_duration(dur: Option<&Duration>) -> String {

    match dur {
        Some(time) => {
            let seconds = time.as_secs() % 60;
            let minutes = (time.as_secs() / 60) % 60;
            let remainder = time.as_millis() % 1000;

            format!("{}:{:02}.{:03}", minutes, seconds, remainder)
        },
        None => {
            "No Time".to_string()
        }
    }
}

