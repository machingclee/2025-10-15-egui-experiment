use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::scripts_folder::Data;
use egui::Ui;

pub fn confirm_delete_folder_window(ui: &mut Ui, folder: &Data) {
    egui::Window::new("Confirm Delete")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.label(format!(
                "Are you sure you want to delete this folder: {}?",
                folder.name
            ));
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    crate::with_folder_state(|state| {
                        *state.folder_to_delete.write().unwrap() = None;
                    });
                }
                if ui.button("Delete").clicked() {
                    dispatch_folder_command(FolderCommand::DeleteFolder {
                        folder_id: folder.id,
                    });
                    crate::with_folder_state(|state| {
                        *state.folder_to_delete.write().unwrap() = None;
                    });
                }
            });
        });
}
