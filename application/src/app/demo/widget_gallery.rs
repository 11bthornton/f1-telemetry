use crate::telemetry_data::car_telemetry_data::{PacketCarTelemetryData, CarTelemetryData};
use crate::PARTICIPANTS;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First,
    Second,
    Third,
}

/// Shows off one example of each major type of widget.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WidgetGallery {
    enabled: bool,
    visible: bool,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,

    #[cfg(feature = "chrono")]
    #[cfg_attr(feature = "serde", serde(skip))]
    date: Option<chrono::Date<chrono::Utc>>,

    #[cfg_attr(feature = "serde", serde(skip))]
    texture: Option<egui::TextureHandle>,
    pub data: Arc<Mutex<PacketCarTelemetryData>>,
    car_index: u16,
    car_name: String,
}

impl Default for WidgetGallery {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            string: Default::default(),
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: false,
            #[cfg(feature = "chrono")]
            date: None,
            texture: None,
            data: Default::default(),
            car_index: 0,
            car_name: String::from("None Selected"),
        }
    }
}

impl super::Demo for WidgetGallery {
    fn name(&self) -> &'static str {
        "Car Telemetry"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }

    fn get_thing(
        &mut self,
    ) -> Option<Arc<Mutex<crate::telemetry_data::car_telemetry_data::PacketCarTelemetryData>>> {
        Some(self.data.clone())
    }
}

impl super::View for WidgetGallery {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.set_visible(self.visible);

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.visible, "Visible")
                .on_hover_text("Uncheck to hide all the widgets.");
            if self.visible {
                ui.checkbox(&mut self.enabled, "Interactive")
                    .on_hover_text("Uncheck to inspect how the widgets look when disabled.");
            }
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            let tooltip_text = "The full egui documentation.\nYou can also click the different widgets names in the left column.";
            ui.hyperlink("https://docs.rs/egui/").on_hover_text(tooltip_text);
            // ui.add(crate::egui_github_link_file!(
            //     "Source code of the widget gallery"
            // ));
        });
    }
}

impl WidgetGallery {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            enabled: _,
            visible: _,
            boolean,
            radio,
            scalar,
            string,
            color,
            animate_progress_bar,
            #[cfg(feature = "chrono")]
            date,
            texture,
            data,
            car_index,
            car_name,
        } = self;

        let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "example",
                egui::ColorImage::example(),
                egui::TextureFilter::Linear,
            )
        });

        ui.label("Select Car");
        
        let mut data : &CarTelemetryData = &Default::default();

        if crate::TELEM.lock().unwrap().is_none() {
            return;
        }

        let c_data = crate::TELEM.lock().unwrap();
        data = &c_data.as_ref().unwrap().telemetry_data[*car_index as usize];

        let combo = egui::ComboBox::from_label("")
            .selected_text(car_name.as_str())
            .width(150.0)
            .show_ui(ui, |ui| {
                if let Some(participants) = (*PARTICIPANTS).lock().unwrap().as_ref() {
                    for (index, participant) in participants.participants.iter().enumerate() {
                        if !participant.name().trim().is_empty() {
                            ui.selectable_value(car_index, index as u16, participant.name());
                        } else {
                            ui.selectable_value(
                                car_index,
                                index as u16,
                                format!("Car {}", index.to_string()),
                            );
                        }
                    }
                }
            });

        if let Some(participants) = (*PARTICIPANTS).lock().unwrap().as_ref() {
            self.car_name = participants.participants[*car_index as usize]
                .name()
                .to_string();
        }

        ui.end_row();

        ui.label("Speed");
        ui.label(data.speed.to_string());
        ui.end_row();

        use egui::special_emojis::GITHUB;

        ui.label("Gear");
        ui.label(data.gear.to_string());
        ui.end_row();

        ui.label("Engine RPM");
        ui.label(data.engine_rpm.to_string());
        ui.end_row();

        ui.label("Throttle");
        let throttle = data.throttle;
        let throttle_bar = egui::ProgressBar::new(throttle)
            .show_percentage()
            .animate(*animate_progress_bar);

        *animate_progress_bar = ui
            .add(throttle_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.label("Brake");
        let brake = data.brake;

        let brake_bar = egui::ProgressBar::new(brake)
            .show_percentage()
            .animate(*animate_progress_bar);

        *animate_progress_bar = ui
            .add(brake_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.label("Clutch");
        ui.label(match data.clutch {
            100 => "Engaged",
            _ => "Disengaged",
        });
        ui.end_row();

        ui.label("DRS");
        ui.label(match data.drs {
            1 => "Engaged",
            _ => "Disengaged",
        });
        ui.end_row();

        ui.label("Front surface temperatures");
        ui.label(format!("{}°C {}°C", data.tyre_surface_temps[2].to_string(), data.tyre_surface_temps[3].to_string()));
        ui.end_row();

        ui.label("Rear surface temperatures");
        ui.label(format!("{}°C {}°C", data.tyre_surface_temps[0].to_string(), data.tyre_surface_temps[1].to_string()));
        ui.end_row();


        ui.label("Front carcass temperatures");
        ui.label(format!("{}°C {}°C", data.tyre_inner_temps[2].to_string(), data.tyre_surface_temps[3].to_string()));
        ui.end_row();

        ui.label("Rear carcass temperatures");
        ui.label(format!("{}°C {}°C", data.tyre_inner_temps[0].to_string(), data.tyre_surface_temps[1].to_string()));
        ui.end_row();
        

        ui.label("Front Left Tyre Pressure");
        ui.label(data.tyre_pressures[0].to_string());
        ui.end_row();

        ui.label("Front Right Tyre Pressure");
        ui.label(data.tyre_pressures[1].to_string());
        ui.end_row();

        ui.label("Rear Left Tyre Pressure");
        ui.label(data.tyre_pressures[2].to_string());
        ui.end_row();

        ui.label("Rear Right Tyre Pressure");
        ui.label(data.tyre_pressures[3].to_string());
        ui.end_row();

        ui.label("Engine Temperature");
        ui.label(data.engine_temp.to_string());
        ui.end_row();

        ui.add(doc_link_label("RadioButton", "radio"));
        ui.horizontal(|ui| {
            ui.radio_value(radio, Enum::First, "First");
            ui.radio_value(radio, Enum::Second, "Second");
            ui.radio_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label(
            "SelectableLabel",
            "selectable_value,SelectableLabel",
        ));
        ui.horizontal(|ui| {
            ui.selectable_value(radio, Enum::First, "First");
            ui.selectable_value(radio, Enum::Second, "Second");
            ui.selectable_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("ComboBox", "ComboBox"));

        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{:?}", radio))
            .show_ui(ui, |ui| {
                ui.selectable_value(radio, Enum::First, "First");
                ui.selectable_value(radio, Enum::Second, "Second");
                ui.selectable_value(radio, Enum::Third, "Third");
            });
        ui.end_row();

        ui.add(doc_link_label("Slider", "Slider"));
        ui.add(egui::Slider::new(scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.add(doc_link_label("DragValue", "DragValue"));
        ui.add(egui::DragValue::new(scalar).speed(1.0));
        ui.end_row();

        ui.add(doc_link_label("Color picker", "color_edit"));
        ui.color_edit_button_srgba(color);
        ui.end_row();

        let img_size = 16.0 * texture.size_vec2() / texture.size_vec2().y;

        ui.add(doc_link_label("Image", "Image"));
        ui.image(texture, img_size);
        ui.end_row();

        ui.add(doc_link_label("ImageButton", "ImageButton"));
        if ui.add(egui::ImageButton::new(texture, img_size)).clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        #[cfg(feature = "chrono")]
        {
            let date = date.get_or_insert_with(|| chrono::offset::Utc::now().date());
            ui.add(doc_link_label("DatePickerButton", "DatePickerButton"));
            ui.add(egui_extras::DatePickerButton::new(date));
            ui.end_row();
        }
    }
}

fn example_plot(ui: &mut egui::Ui) -> egui::Response {
    use egui::plot::{Line, PlotPoints};
    let n = 128;
    let line_points: PlotPoints = (0..=n)
        .map(|i| {
            use std::f64::consts::TAU;
            let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
            [x, x.sin()]
        })
        .collect();
    let line = Line::new(line_points);
    egui::plot::Plot::new("example_plot")
        .height(32.0)
        .data_aspect(1.0)
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    let label = format!("{}:", title);
    let url = format!("https://docs.rs/egui?search={}", search_term);
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(label, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}
