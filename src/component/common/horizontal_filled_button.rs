use crate::component::common::div_with_padding::div_with_padding;
use eframe::epaint::Color32;
use egui::{Response, Ui};

pub fn horizontal_filled_button(
    ui: &mut Ui,
    button_width: f32,
    button_height: f32,
    button_text: String,
    button_id: String,
) -> Response {
    let response = ui
        .vertical_centered(|ui| {
            ui.allocate_ui(egui::vec2(button_width, button_height), |ui| {
                div_with_padding(ui, 4.0, false, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                egui::RichText::new(button_text)
                                    .font(egui::FontId::proportional(13.0)),
                            );
                        })
                    })
                    .response
                });
                let rect = ui.min_rect();
                ui.interact(rect, ui.make_persistent_id(button_id), egui::Sense::click())
            })
            .inner
        })
        .inner;

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);

        // Paint a semi-transparent overlay on hover
        ui.painter().rect_filled(
            response.rect,
            4.0,                                           // corner radius
            Color32::from_rgba_premultiplied(0, 0, 0, 30), // hover color
        );
    }

    response
}
