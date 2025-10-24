use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::prisma::shell_script::Data;
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
                // Manual bold effect by painting text with offset
                let text = &script.name;
                let font_id = egui::FontId::proportional(16.0);
                let color = ui.style().visuals.text_color();
                
                let galley = ui.painter().layout_no_wrap(text.to_string(), font_id.clone(), color);
                let pos = ui.cursor().left_top();
                
                // Paint text multiple times with slight offsets to create bold effect
                for x_offset in [0.0, 0.3, 0.6] {
                    for y_offset in [0.0, 0.3] {
                        ui.painter().galley(
                            pos + egui::vec2(x_offset, y_offset),
                            galley.clone(),
                            color,
                        );
                    }
                }
                
                // Advance the cursor
                ui.advance_cursor_after_rect(galley.rect.translate(pos.to_vec2()));
                
                if ui.button("Rename").clicked() {
                    self.renaming_script_id = Some(script.id);
                    self.renaming_name = script.name.clone();
                }
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
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
                            crate::dispatch_folder_command(
                                crate::domain::folder::folder_command_handler::FolderCommand::DeleteScript {
                                    script_id: script.id,
                                },
                            );
                        }
                    },
                );
            });
            ui.label("Command:");
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
