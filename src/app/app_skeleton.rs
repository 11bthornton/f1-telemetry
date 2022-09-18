use std::sync::{Arc, Mutex};

use crate::app;
use crate::telemetry_data::{
    car_telemetry_data::PacketCarTelemetryData, motion_data::PacketMotionData,
};

use super::TelemetryViewerApp;
pub struct TemplateApp {
    pub telemetry_data: Arc<Mutex<PacketCarTelemetryData>>,
    pub car_motion_data: Arc<Mutex<PacketMotionData>>,
    pub state: State,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            telemetry_data: Arc::new(Mutex::new(PacketCarTelemetryData::default())),
            car_motion_data: Arc::new(Mutex::new(PacketMotionData::default())),
            state: State::default(),
        }
    }
}

impl TemplateApp {
    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        if is_mobile(ui.ctx()) {
            ui.menu_button("⚙️ Options", |ui| {
                ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
            });
        } else {
            ui.toggle_value(&mut self.state.backend_panel.open, "⚙️ Options");
        }

        ui.separator();

        let mut selected_anchor = self.state.selected_anchor.clone();

        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
                if _frame.is_web() {
                    ui.output().open_url(format!("#{}", anchor));
                }
            }
        }
        self.state.selected_anchor = selected_anchor;
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let mut vec = vec![
            (
                "telemetry",
                "telemetry",
                &mut self.state.telemetry_app as &mut dyn eframe::App,
            ),
            (
                "✨ Demos",
                "demo",
                &mut self.state.demo as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "demo".into();
        }
    }
}

#[derive(Default)]
pub struct State {
    backend_panel: app::OptionsPanel,
    pub telemetry_app: TelemetryViewerApp,
    pub demo: DemoApp,
    selected_anchor: String,
}
impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DemoApp {
    pub demo_windows: crate::app::demo::demo_app_windows::DemoWindows,
}

impl eframe::App for TemplateApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state.selected_anchor.is_empty() {
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, _frame);
            });
        });

        self.state.backend_panel.update(ctx, _frame);

        if !is_mobile(ctx)
            && (self.state.backend_panel.open || ctx.memory().everything_is_visible())
        {
            egui::SidePanel::left("backend_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("⚙️ Options");
                    });
                });
        }

        self.show_selected_app(ctx, _frame);
        ctx.request_repaint();
    }
}

pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.input().screen_rect().size();
    screen_size.x < 550.0
}
