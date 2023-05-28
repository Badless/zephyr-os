pub fn about(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("About").open(open).show(ctx, |ui| {
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
            ui.label(egui::RichText::new("ZephyrOS 1.0").font(egui::FontId::proportional(32.0)));
            ui.label(
                egui::RichText::new(
                    "Zephyr OS is a slick and feature packed OS
                        which is written in rust (compiled to WASM)
                        Includes a fully working package manager called Zinc",
                )
                .font(egui::FontId::proportional(20.0)),
            );
            ui.separator();
            ui.hyperlink("https://egui.rs/");
            ui.hyperlink("https://trunkrs.dev");
        });
    });
}

pub fn settings(
    ctx: &egui::Context,
    open: &mut bool,
    bar_width: &mut f32,
    bar_x: &mut f32,
    bar_y: &mut f32,
) {
    egui::Window::new("Settings").open(open).show(ctx, |ui| {
        ui.collapsing("egui", |ui| {
            ctx.style_ui(ui);
        });
        ui.collapsing("ZephyrOS", |ui| {
            ui.label("bar width:");
            ui.add(egui::Slider::new(
                bar_width,
                520.0..=ctx.screen_rect().size().x - 30.0,
            ));
            ui.label("bar x:");
            ui.add(egui::Slider::new(
                bar_x,
                0.0..=ctx.screen_rect().size().x - *bar_width - 10.0,
            ));
            ui.label("bar y:");
            ui.add(egui::Slider::new(
                bar_y,
                0.0..=-ctx.screen_rect().size().y + 70.0,
            ));
        });
    });
}

pub fn text_editor(ctx: &egui::Context, open: &mut bool, text: &mut String) {
    egui::Window::new("Text Editor").open(open).show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                // download text as .txt file or something
                // std::fs::File doesnt work btw...
            }
        });
        ui.add(
            egui::TextEdit::multiline(text)
                .desired_width(ctx.screen_rect().size().x / 3.)
                .desired_rows(20),
        );
    });
}
