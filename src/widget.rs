use chrono::Timelike;

pub fn clock(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("clock")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::ZERO)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .open(open)
        .show(ctx, |ui| {
            let clock: f64 = chrono::Local::now().time().num_seconds_from_midnight() as f64;

            ui.label(
                egui::RichText::new(format!(
                    "{:02}:{:02}:{:02}",
                    (clock % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                    (clock % (60.0 * 60.0) / 60.0).floor(),
                    (clock % 60.0).floor(),
                ))
                .font(egui::FontId::proportional(32.0)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(egui::RichText::new("nickname").font(egui::FontId::proportional(14.0)));
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if ui
                        .button(
                            egui::RichText::new("Log Out!").font(egui::FontId::proportional(14.0)),
                        )
                        .clicked()
                    {
                        // Change TemplateApp into LoginScreen.
                        Box::new(|cc| Box::new(crate::LoginScreen::new(cc)));
                    };
                });
            });
        });
}

pub fn menu_start(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("menu_start")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::ZERO)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .open(open)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Start Menu").font(egui::FontId::proportional(32.0)));
            if ui
                .button(egui::RichText::new("Welcome").font(egui::FontId::proportional(24.0)))
                .clicked()
            {
                // Open about window
            };
        });
}

/*
    If you can help We really want widget system to work like:

    widget::new(ctx, &mut bool) {
        ui stuff
    };

    so if you can make it We will be so happy! Thanks! fuck off
*/
