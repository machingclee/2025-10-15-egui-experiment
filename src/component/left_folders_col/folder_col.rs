use crate::component::left_folders_col::folder_item::FolderItem;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use egui::Ui;
use std::sync::Arc;

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
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(format!("{}", "Scripts Folders"))
                        .strong()
                        .font(egui::FontId::proportional(16.0)),
                );
                ui.separator();
                ui.add_space(10.0);

                Self::add_folder_button(ui);

                ui.add_space(10.0);

                Self::folders(ui);
            });
    }

    fn folders(ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Get direct access to state - we handle locking ourselves!
            crate::with_folder_state(|state| {
                let folders_vec = (*state.folder_list.read().unwrap()).clone();st
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
                        let mut folder_item = FolderItem::new(folder, selected_id, display_name);
                        folder_item.view(ui);
                    }
                }
            });
        });
    }

    pub fn add_folder_button(ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            let response = ui.button(egui::RichText::new("Add Folder").font(egui::FontId::proportional(18.0)));
            if response.clicked() {
                dispatch_folder_command(FolderCommand::CreateFolder {});
            }
        });
    }
}
