use std::sync::Arc;

use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;

pub struct FolderColumn;

impl FolderColumn {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, ctx: &egui::Context) {
        egui::SidePanel::left("Folders Panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=600.0)
            .show(ctx, |ui| {
                ui.label("Scripts Folders");
                ui.separator();

                ui.vertical_centered(|ui| {
                    let response = ui.button("add folder");
                    if response.clicked() {
                        dispatch_folder_command(FolderCommand::CreateFolder {});
                    }
                });

                ui.add_space(10.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Get direct access to state - we handle locking ourselves!
                    let state = crate::get_folder_state_ref();
                    let folders_vec = (*state.folder_list.read().unwrap()).clone();
                    let selected_id = *state.selected_folder_id.read().unwrap();
                    let rename_folder = state.folder_to_rename.read().unwrap().as_ref().cloned();
                    let rename_text = state.rename_text.read().unwrap().as_ref().cloned();

                    if folders_vec.is_empty() {
                        ui.label("No folders yet...");
                    } else {
                        for folder in &*folders_vec {
                            let is_renaming = rename_folder
                                .as_ref()
                                .map(|f| f.id == folder.id)
                                .unwrap_or(false);
                            let display_name = if is_renaming {
                                rename_text.as_ref().unwrap_or(&folder.name)
                            } else {
                                &folder.name
                            };
                            let mut folder_item =
                                FolderItem::new(folder, selected_id, display_name);
                            folder_item.view(ui);
                        }
                    }
                });
            });
    }
}

struct FolderItem<'a> {
    folder: &'a crate::prisma::scripts_folder::Data,
    selected_id: Option<i32>,
    display_name: &'a str,
}

impl<'a> FolderItem<'a> {
    fn new(
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

    fn view(&mut self, ui: &mut egui::Ui) {
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
                    let response = ui.selectable_label(is_selected, self.display_name);
                    if response.clicked() {
                        dispatch_folder_command(FolderCommand::SelectFolder { id: self.folder.id });
                    }
                    response
                },
            );

            self.dots_menu(ui, self.folder);
        });
    }
    fn dots_menu(&mut self, ui: &mut egui::Ui, folder: &crate::prisma::scripts_folder::Data) {
        let state = crate::get_folder_state_ref();

        let delete_folder = state.folder_to_delete.read().unwrap().as_ref().cloned();
        let rename_folder = state.folder_to_rename.read().unwrap().as_ref().cloned();

        ui.menu_button("...", |ui| {
            if ui
                .add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.button("Rename Folder")
                })
                .clicked()
            {
                let folder_ = Arc::new(folder.clone());
                *state
                    .folder_to_rename
                    .write()
                    .unwrap_or_else(|e| e.into_inner()) = Some(folder_.clone());
                *state.rename_text.write().unwrap_or_else(|e| e.into_inner()) =
                    Some(folder_.name.clone());
            }
            if ui
                .add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.button("Delete Folder")
                })
                .clicked()
            {
                let folder_ = Arc::new(folder.clone());
                *state
                    .folder_to_delete
                    .write()
                    .unwrap_or_else(|e| e.into_inner()) = Some(folder_);
            }
        });

        // Show delete confirmation if this folder is selected for deletion
        if let Some(folder_) = delete_folder
            && folder_.id == folder.id
        {
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
                            *state.folder_to_delete.write().unwrap() = None;
                        }
                        if ui.button("Delete").clicked() {
                            dispatch_folder_command(FolderCommand::DeleteFolder { id: folder.id });
                            *state.folder_to_delete.write().unwrap() = None;
                        }
                    });
                });
        }

        if let Some(folder_) = rename_folder
            && folder_.id == folder.id
        {
            egui::Window::new("Rename Folder")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("Input new folder name:");
                    ui.add_space(10.0);
                    let mut text = state
                        .rename_text
                        .read()
                        .unwrap()
                        .as_ref()
                        .cloned()
                        .unwrap_or_default();
                    ui.text_edit_singleline(&mut text);
                    *state.rename_text.write().unwrap_or_else(|e| e.into_inner()) =
                        Some(text.clone());
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            *state.folder_to_rename.write().unwrap() = None;
                            *state.rename_text.write().unwrap() = None;
                        }
                        if ui.button("Rename").clicked() {
                            dispatch_folder_command(FolderCommand::RenameFolder {
                                folder_id: folder_.id,
                                new_name: text,
                            });
                            *state.folder_to_rename.write().unwrap() = None;
                            *state.rename_text.write().unwrap() = None;
                        }
                    });
                });
        }
    }
}
