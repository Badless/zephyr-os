pub fn clock(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("clock")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::ZERO)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .open(open)
        .show(ctx, |ui| {
            ui.label("Coming Soon!");
        });
}

pub fn settings(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("Settings")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::ZERO)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .open(open)
        .show(ctx, |ui| {
            ctx.style_ui(ui);
        });
}

/*
    If you can help We really want widget system to work like:

    widget::new(ctx, &mut bool) {
        ui stuff
    };

    so if you can make it We will be so happy! Thanks!
*/
