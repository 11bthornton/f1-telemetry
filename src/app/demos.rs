use egui::{Context, ScrollArea, Ui};
use std::collections::BTreeSet;

use crate::app::app_skeleton::is_mobile;
use crate::app::demo::Demo;
use crate::app::demo::View;

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Demos {
    #[cfg_attr(feature = "serde", serde(skip))]
    demos: Vec<Box<dyn Demo>>,

    open: BTreeSet<String>,
}

impl Default for Demos {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::new(crate::app::demo::context_menu::ContextMenus::default()),
            Box::new(crate::app::demo::dancing_strings::DancingStrings::default()),
            Box::new(crate::app::demo::drag_and_drop::DragAndDropDemo::default()),
            // Box::new(crate::app::demo::font_book::FontBook::default()),
            Box::new(crate::app::demo::MiscDemoWindow::default()),
            Box::new(crate::app::demo::multi_touch::MultiTouch::default()),
            Box::new(crate::app::demo::painting::Painting::default()),
            Box::new(crate::app::demo::plot_demo::PlotDemo::default()),
            Box::new(crate::app::demo::scrolling::Scrolling::default()),
            Box::new(crate::app::demo::sliders::Sliders::default()),
            Box::new(crate::app::demo::strip_demo::StripDemo::default()),
            Box::new(crate::app::demo::table_demo::TableDemo::default()),
            Box::new(crate::app::demo::text_edit::TextEdit::default()),
            // Box::new(super::widget_gallery::WidgetGallery::default()),
            Box::new(crate::app::demo::window_options::WindowOptions::default()),
            Box::new(crate::app::demo::tests::WindowResizeTest::default()),
            Box::new(crate::app::demo::window_with_panels::WindowWithPanels::default()),
        ])
    }
}

impl Demos {
    pub fn from_demos(demos: Vec<Box<dyn Demo>>) -> Self {
        let mut open = BTreeSet::new();
        // open.insert(
        //     super::widget_gallery::WidgetGallery::default()
        //         .name()
        //         .to_owned(),
        // );

        Self { demos, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            ui.toggle_value(&mut is_open, demo.name());
            set_open(open, demo.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Tests {
    #[cfg_attr(feature = "serde", serde(skip))]
    demos: Vec<Box<dyn Demo>>,

    open: BTreeSet<String>,
}

impl Default for Tests {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::new(crate::app::demo::tests::CursorTest::default()),
            Box::new(crate::app::demo::tests::IdTest::default()),
            Box::new(crate::app::demo::tests::InputTest::default()),
            Box::new(crate::app::demo::layout_test::LayoutTest::default()),
            Box::new(crate::app::demo::tests::ManualLayoutTest::default()),
            Box::new(crate::app::demo::tests::TableTest::default()),
        ])
    }
}

impl Tests {
    pub fn from_demos(demos: Vec<Box<dyn Demo>>) -> Self {
        let mut open = BTreeSet::new();
        // open.insert(
        //     super::widget_gallery::WidgetGallery::default()
        //         .name()
        //         .to_owned(),
        // );

        Self { demos, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            ui.toggle_value(&mut is_open, demo.name());
            set_open(open, demo.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindows {
    about_is_open: bool,
    demos: Demos,
    tests: Tests,
}

impl Default for DemoWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            demos: Default::default(),
            tests: Default::default(),
        }
    }
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        if self.about_is_open {
        } else {
            self.mobile_top_bar(ctx);
            self.show_windows(ctx);
        }
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let font_size = 20.0;

                ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
                    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                    self.demo_list_ui(ui);
                    if ui.ui_contains_pointer() && ui.input().pointer.any_click() {
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    use egui::special_emojis::{GITHUB, TWITTER};
                    ui.hyperlink_to(
                        egui::RichText::new(TWITTER).size(font_size),
                        "https://twitter.com/ernerfeldt",
                    );
                    ui.hyperlink_to(
                        egui::RichText::new(GITHUB).size(font_size),
                        "https://github.com/emilk/egui",
                    );
                });
            });
        });
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(145.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{} egui on GitHub", GITHUB),
                    "https://github.com/emilk/egui",
                );
                ui.hyperlink_to(
                    format!("{} @ernerfeldt", TWITTER),
                    "https://twitter.com/ernerfeldt",
                );

                ui.separator();

                self.demo_list_ui(ui);
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu_button(ui);
            });
        });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.demos.windows(ctx);
        self.tests.windows(ctx);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                // ui.toggle_value(&mut self.about_is_open, self.about.name());

                ui.separator();
                self.demos.checkboxes(ui);
                ui.separator();
                self.tests.checkboxes(ui);
                ui.separator();

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory().reset_areas();
                }
            });
        });
    }
}

// ----------------------------------------------------------------------------

fn file_menu_button(ui: &mut Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("Organize windows").clicked() {
            ui.ctx().memory().reset_areas();
            ui.close_menu();
        }
        if ui
            .button("Reset egui memory")
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            *ui.ctx().memory() = Default::default();
            ui.close_menu();
        }
    });
}
