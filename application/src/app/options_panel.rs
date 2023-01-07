#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct OptionsPanel {
    pub open: bool,
}

impl Default for OptionsPanel {
    fn default() -> Self {
        Self { open: false }
    }
}

impl OptionsPanel {
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.separator();
    }
}

// ----------------------------------------------------------------------------
