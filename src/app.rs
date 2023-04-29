use chrono::Timelike;
use egui_notify::Toasts;

use crate::widget;
use crate::window;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    window_about: bool,
    window_settings: bool,
    widget_clock: bool,
    widget_menu: bool,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    notify: Toasts,
    #[serde(skip)]
    wallpaper: egui_extras::RetainedImage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            window_about: true,
            window_settings: false,
            widget_clock: false,
            widget_menu: false,
            notify: Toasts::default(),
            wallpaper: egui_extras::RetainedImage::from_image_bytes(
                "images/WebStorm.jpg",
                include_bytes!("images/WebStorm.jpg"),
            )
            .unwrap(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom-bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::ImageButton::new(
                        egui_extras::RetainedImage::from_image_bytes(
                            "images/zephyrosbar.png",
                            include_bytes!("images/zephyrosbar.png"),
                        )
                        .unwrap()
                        .texture_id(ctx),
                        [32.0, 32.0],
                    ))
                    .clicked()
                {
                    if self.widget_menu {
                        self.widget_menu = false;
                    } else {
                        self.widget_menu = true;
                    }
                }
                ui.add(egui::Label::new("Search..."));

                let clock: f64 = chrono::Local::now().time().num_seconds_from_midnight() as f64;
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(format!(
                            "{:02}:{:02}",
                            (clock % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                            (clock % (60.0 * 60.0) / 60.0).floor(),
                        ))
                        .clicked()
                    {
                        if self.widget_clock {
                            self.widget_clock = false;
                        } else {
                            self.widget_clock = true;
                        }
                    };

                    //ui.add(super::toggle_switch::toggle(&mut self.widget_clock));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add(egui::Image::new(
                self.wallpaper.texture_id(ctx),
                [ui.available_width(), ui.available_height()],
            ));
        });

        // We need better widget system, look at widget.rs
        widget::clock(ctx, &mut self.widget_clock);
        widget::menu_start(ctx, &mut self.widget_menu);

        window::about(ctx, &mut self.window_about);
        window::settings(ctx, &mut self.window_settings);

        self.notify.show(ctx);
    }
}
