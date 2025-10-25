use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::prisma::shell_script::Data;
use eframe::epaint::Color32;
use egui::Ui;

impl ScriptsColumn {
    pub fn scripts_of_selected_folder(&mut self, ui: &mut Ui) {
        crate::with_folder_state(|state| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                crate::component::right_scripts_col::scripts_col::with_scritps_from_selected_folder(
                    |scripts| {
                        for script in scripts.iter() {
                            self.script_item(ui, &script);
                        }
                    },
                );
            });
        })
    }

    fn script_item(&mut self, ui: &mut Ui, script: &&Data) {
        let frame = egui::Frame::group(ui.style()).fill(ui.visuals().faint_bg_color);
        let frame_response = frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Use built-in bold font
                ui.label(egui::RichText::new(&script.name).strong().size(16.0));

                if ui.button("Rename").clicked() {
                    self.renaming_script_id = Some(script.id);
                    self.renaming_name = script.name.clone();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Execute").clicked() {
                        // Execute the script command
                        crate::run_terminal_command(script.command.clone());
                    }
                    if ui.button("Edit").clicked() {
                        self.editing_script_id = Some(script.id);
                        self.editing_command = script.command.clone();
                    }
                    if ui.button("Copy").clicked() {
                        ui.ctx().copy_text(script.command.clone());
                    }

                    if ui.button("Delete").clicked() {
                        self.script_to_delete = Some(script.id);
                    }
                });
            });
            ui.label("Command:");
            ui.add_space(2.0);
            egui::Frame::NONE
                .fill(ui.visuals().code_bg_color)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut script.command.clone())
                            .font(egui::TextStyle::Monospace)
                            .interactive(false)
                            .frame(false)
                            .desired_rows(5)
                            .desired_width(f32::INFINITY),
                    );
                });
        });
        let response = &frame_response.response;
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            // Paint a semi-transparent overlay on hover
            ui.painter().rect_filled(
                response.rect,
                4.0,                                           // corner radius
                Color32::from_rgba_premultiplied(0, 0, 0, 30), // hover color
            );
        }
        // Check for double-click on the frame background
        if ui.input(|i| {
            i.pointer
                .button_double_clicked(egui::PointerButton::Primary)
        }) {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if frame_response.response.rect.contains(pos) {
                    crate::run_terminal_command(script.command.clone());
                }
            }
        }

        ui.add_space(10.0);
    }
}
