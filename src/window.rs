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
        });
    });
}

pub fn settings(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("Settings").open(open).show(ctx, |ui| {
        ctx.style_ui(ui);
    });
}
