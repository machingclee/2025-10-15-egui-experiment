use egui::{Context, Ui};

pub fn top_menu(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                quit_button(ctx, ui);
            });
            ui.add_space(16.0);
            // egui::widgets::g lobal_theme_preference_buttons(ui);
        });
    });
}

fn quit_button(ctx: &Context, ui: &mut Ui) {
    if ui.button("Quit").clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}
