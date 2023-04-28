use egui_notify::Toasts;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    window_about: bool,
    window_settings: bool,

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
                    self.window_settings = true;
                }
                ui.add(egui::Label::new("Search..."));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add(egui::Image::new(
                self.wallpaper.texture_id(ctx),
                [ui.available_width(), ui.available_height()],
            ));
        });

        egui::Window::new("About")
            .open(&mut self.window_about)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
				if ctx.style().visuals.dark_mode {
                    ui.add(egui::Image::new(
                        egui_extras::RetainedImage::from_image_bytes(
                            "images/zephyrostrans.png",
                            include_bytes!("images/zephyrostrans.png"),
                        )
                        .unwrap()
                        .texture_id(ctx),
                        [320.0, 320.0],
                    ));
				} else {
					ui.add(egui::Image::new(
                        egui_extras::RetainedImage::from_image_bytes(
                            "images/zephyrostransdark.png",
                            include_bytes!("images/zephyrostransdark.png"),
                        )
                        .unwrap()
                        .texture_id(ctx),
                        [320.0, 320.0],
                    ));
				}
                    ui.label(
                        egui::RichText::new("ZephyrOS 1.0").font(egui::FontId::proportional(32.0)),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Zephyr OS is a slick and feature packed OS
                        which is written in rust (compiled to WASM)
                        Includes a fully working package manager called Zinc",
                        )
                        .font(egui::FontId::proportional(20.0)),
                    );
                });
            });

        egui::Window::new("Settings")
            .open(&mut self.window_settings)
            .show(ctx, |ui| {
                ctx.style_ui(ui);
            });

        self.notify.show(ctx);
    }
}
