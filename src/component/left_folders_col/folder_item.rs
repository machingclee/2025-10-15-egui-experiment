use crate::component::left_folders_col::confirm_delete_folder_window::confirm_delete_folder_window;
use crate::component::left_folders_col::rename_folder_window::rename_folder_window;
use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::scripts_folder::Data;
use crate::{dispatch_folder_command, with_folder_state_reducer};
use egui::Ui;
use std::sync::Arc;

pub struct FolderItem<'a> {
    folder: &'a crate::prisma::scripts_folder::Data,
    selected_id: Option<i32>,
    display_name: &'a str,
}

impl<'a> FolderItem<'a> {
    pub fn new(
        folder: &'a crate::prisma::scripts_folder::Data,
        selected_id: Option<i32>,
        display_name: &'a str,
    ) -> Self {
        Self {
            folder,
            selected_id,
            display_name,
        }
    }

    pub fn view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let is_selected = self.selected_id == Some(self.folder.id);

            // Calculate space for label (available width minus estimated menu space)
            let available_width = ui.available_width();
            let dots_menu_width = 40.0; // Estimate for menu button
            let label_width = (available_width - dots_menu_width).max(0.0);

            // Make label expand to fill calculated space
            ui.add_sized(
                [label_width, ui.available_height() + 5.0],
                |ui: &mut egui::Ui| {
                    let mut response = None;
                    ui.horizontal(|ui| {
                        response = Some(ui.selectable_label(is_selected, self.display_name));
                        ui.allocate_space(ui.available_size());
                    });
                    let response = response.unwrap();
                    if response.clicked() {
                        with_folder_state_reducer(|reducer| {
                            reducer.set_scripts_of_selected_folder(vec![])
                        });
                        dispatch_folder_command(FolderCommand::SelectFolder {
                            folder_id: self.folder.id,
                        });
                    }
                    response
                },
            );

            self.dots_menu(ui, self.folder);
        });
    }
    fn dots_menu(&mut self, ui: &mut egui::Ui, folder: &crate::prisma::scripts_folder::Data) {
        let (delete_folder, rename_folder) = crate::with_folder_state(|state| {
            let delete_folder = state.folder_to_delete.read().unwrap().as_ref().cloned();
            let rename_folder = state.folder_to_rename.read().unwrap().as_ref().cloned();
            (delete_folder, rename_folder)
        });

        ui.menu_button("...", |ui| {
            if ui
                .add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.button("Rename Folder")
                })
                .clicked()
            {
                let folder_ = Arc::new(folder.clone());
                crate::with_folder_state(|state| {
                    *state.folder_to_rename.write().unwrap() = Some(folder_.clone());
                    *state.rename_text.write().unwrap() = Some(folder_.name.clone());
                });
            }
            if ui
                .add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.button("Delete Folder")
                })
                .clicked()
            {
                let folder_ = Arc::new(folder.clone());
                crate::with_folder_state(|state| {
                    *state.folder_to_delete.write().unwrap() = Some(folder_);
                });
            }
        });

        // Show delete confirmation if this folder is selected for deletion
        if let Some(folder_) = delete_folder
            && folder_.id == folder.id
        {
            confirm_delete_folder_window(ui, folder);
        }

        if let Some(folder_) = rename_folder
            && folder_.id == folder.id
        {
            rename_folder_window(ui, folder_);
        }
    }
}
