use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use egui::Ui;

impl ScriptsColumn {
    pub fn confirm_delete_script_window(&mut self, ui: &mut Ui, script_id: i32) {
        // Get the script data to display its name
        crate::component::right_scripts_col::scripts_col::with_scritps_from_selected_folder(
            |scripts| {
                if let Some(script) = scripts.iter().find(|s| s.id == script_id) {
                    egui::Window::new("Confirm Delete")
                        .collapsible(false)
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                        .show(ui.ctx(), |ui| {
                            ui.label(format!(
                                "Are you sure you want to delete this script: \"{}\"?",
                                script.name
                            ));
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                if ui.button("Cancel").clicked() {
                                    self.script_to_delete = None;
                                }
                                if ui.button("Delete").clicked() {
                                    dispatch_folder_command(FolderCommand::DeleteScript {
                                        script_id,
                                    });
                                    self.script_to_delete = None;
                                }
                            });
                        });
                }
            },
        );
    }
}

