use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::shell_script::Data;
use egui::Ui;

pub fn confirm_delete_script_window(ui: &mut Ui, script: &Data, on_cancel: impl FnOnce(), on_confirm: impl FnOnce()) {
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
                    on_cancel();
                }
                if ui.button("Delete").clicked() {
                    dispatch_folder_command(FolderCommand::DeleteScript {
                        script_id: script.id,
                    });
                    on_confirm();
                }
            });
        });
}

