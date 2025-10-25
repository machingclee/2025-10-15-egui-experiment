use egui::Response;

pub fn div_with_padding(
    ui: &mut egui::Ui,
    padding: f32,
    is_selected: bool,
    add_contents: impl FnOnce(&mut egui::Ui) -> Response,
) -> Response {
    ui.allocate_ui(ui.available_size(), |ui| {
        egui::Frame::new()
            .fill(if is_selected {
                ui.visuals().selection.bg_fill
            } else {
                ui.visuals().window_fill()
            })
            .stroke(ui.visuals().window_stroke())
            .corner_radius(4.0)
            .inner_margin(padding)
            .show(ui, |ui| {
                let response = add_contents(ui);
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                response
            })
    })
    .inner
    .inner
}
