use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::scripts_folder::Data;
use crate::{dispatch_folder_command, dispatch_folder_command_with_callback};
use egui::Ui;
use std::sync::Arc;

pub fn rename_folder_window(ui: &mut Ui, folder_: Arc<Data>) {
    egui::Window::new("Rename Folder")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.label("Input new folder name:");
            ui.add_space(10.0);
            let mut text = crate::with_folder_state(|state| {
                state
                    .rename_text
                    .read()
                    .unwrap()
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
            });
            ui.text_edit_singleline(&mut text);
            crate::with_folder_state(|state| {
                *state.rename_text.write().unwrap() = Some(text.clone());
            });
            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    crate::with_folder_state(|state| {
                        *state.folder_to_rename.write().unwrap() = None;
                        *state.rename_text.write().unwrap() = None;
                    });
                }
                if ui.button("Rename").clicked() || enter_pressed {
                    dispatch_folder_command_with_callback(
                        FolderCommand::RenameFolder {
                            folder_id: folder_.id,
                            new_name: text,
                        },
                        Some(|| {
                            crate::with_folder_state(|state| {
                                *state.folder_to_rename.write().unwrap() = None;
                                *state.rename_text.write().unwrap() = None;
                            });
                        }),
                    );
                }
            });
        });
}
