use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use egui::Ui;

impl ScriptsColumn {
    pub fn edit_script_window(&mut self, ui: &mut Ui, script_id: i32) {
        egui::Window::new("Edit Script")
            .collapsible(false)
            .resizable(true)
            .default_height(400.0)
            .default_width(600.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.editing_command)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(20)
                        .desired_width(580.0),
                );
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.editing_script_id = None;
                    }
                    if ui.button("Save").clicked() {
                        dispatch_folder_command(FolderCommand::UpdateScript {
                            script_id,
                            new_command: self.editing_command.clone(),
                        });
                        self.editing_script_id = None;
                    }
                });
            });
    }
}
