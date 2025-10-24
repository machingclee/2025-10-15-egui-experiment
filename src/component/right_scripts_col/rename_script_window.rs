use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use egui::Ui;

impl ScriptsColumn {
    pub fn rename_script_window(&mut self, ui: &mut Ui, script_id: i32) {
        egui::Window::new("Rename Script")
            .collapsible(false)
            .resizable(true)
            .default_height(200.0)
            .default_width(400.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.label("New name:");
                ui.text_edit_singleline(&mut self.renaming_name);
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.renaming_script_id = None;
                    }
                    if ui.button("Rename").clicked() {
                        dispatch_folder_command(FolderCommand::UpdateScriptName {
                            script_id,
                            new_name: self.renaming_name.clone(),
                        });
                        self.renaming_script_id = None;
                    }
                });
            });
    }
}
