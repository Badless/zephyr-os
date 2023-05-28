use chrono::Timelike;

pub fn clock(
    ctx: &egui::Context,
    open: &mut bool,
    logged_in: &mut bool,
    nickname: String,
    bar_width: &mut f32,
    bar_x: &mut f32,
    bar_y: &mut f32,
) {
    egui::Window::new("clock")
        .anchor(
            egui::Align2::RIGHT_BOTTOM,
            egui::Vec2::new(
                *bar_width - ctx.screen_rect().size().x + *bar_x + 10.,
                if *bar_y <= -ctx.screen_rect().size().y / 2.0 - 50. {
                    *bar_y + 67.
                    //       ^^ change everytime clock widget is higher.
                } else {
                    *bar_y - 70.
                },
            ),
        )
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
                ui.label(egui::RichText::new(nickname).font(egui::FontId::proportional(14.0)));
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if ui
                        .button(
                            egui::RichText::new("Log Out!").font(egui::FontId::proportional(14.0)),
                        )
                        .clicked()
                    {
                        *logged_in = false;
                    };
                });
            });
        });
}

pub fn menu_start(
    ctx: &egui::Context,
    open: &mut bool,
    app: &mut bool,
    app2: &mut bool,
    app3: &mut bool,
    bar_x: &mut f32,
    bar_y: &mut f32,
) {
    egui::Window::new("menu_start")
        .anchor(
            egui::Align2::LEFT_BOTTOM,
            egui::Vec2::new(
                *bar_x,
                if *bar_y <= -ctx.screen_rect().size().y / 2.0 - 50. {
                    *bar_y + 140.
                    //       ^^^ change everytime menu is higher
                    //       if its new button add 30 to it.
                } else {
                    *bar_y - 70.
                },
            ),
        )
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
                if !*app {
                    *app = true;
                } else {
                    *app = false;
                }
            };
            if ui
                .button(egui::RichText::new("Settings").font(egui::FontId::proportional(24.0)))
                .clicked()
            {
                if !*app2 {
                    *app2 = true;
                } else {
                    *app2 = false;
                }
            };
            if ui
                .button(egui::RichText::new("Text Editor").font(egui::FontId::proportional(24.0)))
                .clicked()
            {
                if !*app3 {
                    *app3 = true;
                } else {
                    *app3 = false;
                }
            }
        });
}

pub fn bar(
    ctx: &egui::Context,
    bool1: &mut bool,
    bool2: &mut bool,
    app1: &mut bool,
    bar_width: &mut f32,
    bar_x: &mut f32,
    bar_y: &mut f32,
) {
    egui::Window::new("bar")
        .anchor(
            egui::Align2::LEFT_BOTTOM,
            egui::Vec2::new(*bar_x, *bar_y - 10.0),
        )
        .fixed_size([*bar_width, 0.0])
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .open(&mut true)
        .show(ctx, |ui| {
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
                    if *bool1 {
                        *bool1 = false;
                    } else {
                        *bool1 = true;
                    }
                }
                ui.add(egui::Label::new("Search..."));
                ui.separator();
                if ui
                    .add(
                        egui::ImageButton::new(
                            egui_extras::RetainedImage::from_image_bytes(
                                "images/settings.png",
                                include_bytes!("images/settings.png"),
                            )
                            .unwrap()
                            .texture_id(ctx),
                            [32.0, 32.0],
                        )
                        .frame(true),
                    )
                    .clicked()
                {
                    if !*app1 {
                        *app1 = true;
                    } else {
                        *app1 = false;
                    }
                }
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
                        if *bool2 {
                            *bool2 = false;
                        } else {
                            *bool2 = true;
                        }
                    };
                    ui.separator();
                });
            });
        });
}

/*
    If you can help We really want widget system to work like:

    widget::new(ctx, &mut bool) {
        ui stuff
    };

    so if you can make it We will be so happy! Thanks!
*/
