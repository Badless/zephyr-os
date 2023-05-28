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
    window_textedit: bool,
    widget_clock: bool,
    widget_menu: bool,
    bar_width: f32,
    bar_x: f32,
    bar_y: f32,
    logged_in: bool,
    nickname: String,
    text: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    notify: Toasts,
    #[serde(skip)]
    wallpaper: egui_extras::RetainedImage,
    #[serde(skip)]
    wallpaper_login: egui_extras::RetainedImage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            window_about: true,
            window_settings: false,
            window_textedit: false,
            widget_clock: false,
            widget_menu: false,
            bar_width: 520.0,
            bar_x: 10.0,
            bar_y: 0.0,
            logged_in: false,
            nickname: "".to_owned(),
            text: "".to_owned(),
            notify: Toasts::default(),
            wallpaper: egui_extras::RetainedImage::from_image_bytes(
                "images/WebStorm.jpg",
                include_bytes!("images/WebStorm.jpg"),
            )
            .unwrap(),
            wallpaper_login: egui_extras::RetainedImage::from_image_bytes(
                "images/WebStormLogin.png",
                include_bytes!("images/WebStormLogin.png"),
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

        if self.logged_in {
            widget::bar(
                ctx,
                &mut self.widget_menu,
                &mut self.widget_clock,
                &mut self.window_settings,
                &mut self.bar_width,
                &mut self.bar_x,
                &mut self.bar_y,
            );

            egui::Area::new("wallpaper")
                .order(egui::Order::Background)
                .show(ctx, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    ui.add(egui::Image::new(
                        self.wallpaper.texture_id(ctx),
                        ctx.screen_rect().size(),
                    ));
                });

            // We need better widget system, look at widget.rs
            widget::clock(
                ctx,
                &mut self.widget_clock,
                &mut self.logged_in,
                self.nickname.to_owned(),
                &mut self.bar_width,
                &mut self.bar_x,
                &mut self.bar_y,
            );
            widget::menu_start(
                ctx,
                &mut self.widget_menu,
                &mut self.window_about,
                &mut self.window_settings,
                &mut self.window_textedit,
                &mut self.bar_x,
                &mut self.bar_y,
            );

            window::about(ctx, &mut self.window_about);
            window::settings(
                ctx,
                &mut self.window_settings,
                &mut self.bar_width,
                &mut self.bar_x,
                &mut self.bar_y,
            );
            window::text_editor(ctx, &mut self.window_textedit, &mut self.text);

            self.notify.show(ctx);
        } else {
            egui::Area::new("wallpaper-login")
                .order(egui::Order::Background)
                .show(ctx, |ui| {
                    ui.add(egui::Image::new(
                        self.wallpaper_login.texture_id(ctx),
                        ctx.screen_rect().size(),
                    ));
                });

            egui::Window::new("login-form")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .title_bar(false)
                .movable(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        let clock: f64 =
                            chrono::Local::now().time().num_seconds_from_midnight() as f64;
                        ui.label(
                            egui::RichText::new(format!(
                                "{:02}:{:02}",
                                (clock % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                                (clock % (60.0 * 60.0) / 60.0).floor(),
                            ))
                            .font(egui::FontId::proportional(32.0)),
                        );
                        ui.separator();
                        ui.add(
                            egui::TextEdit::singleline(&mut self.nickname).hint_text("Nickname"),
                        );
                        ui.separator();
                        if ui.button("Log In!").clicked() {
                            if self.nickname.is_empty() {
                                self.logged_in = false;
                                self.notify.info("Nickname can't be empty!");
                            } else if self.nickname.len() == 1 || self.nickname.len() == 2 {
                                self.logged_in = false;
                                self.notify.info("Nickname must be at least 3 letter long!");
                            } else if self.nickname.contains(" ") {
                                self.logged_in = false;
                                self.notify.info("Nickname can't have spaces!");
                            } else {
                                self.logged_in = true;
                            }
                        }
                    });
                });
            self.notify.show(ctx);
        }
    }
}
