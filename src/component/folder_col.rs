use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;

pub fn folder_col(ctx: &egui::Context) {
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
                    dispatch_folder_command(FolderCommand::CreateFolderCommand {});
                }
            });

            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Get direct access to state - we handle locking ourselves!
                let state = crate::get_folder_state_ref();
                let folders = state.folder_list.read().unwrap();
                let selected_id = *state.selected_folder_id.read().unwrap();

                if folders.is_empty() {
                    ui.label("No folders yet...");
                } else {
                    for folder in &**folders {
                        let is_selected = selected_id == Some(folder.id);

                        ui.horizontal(|ui| {
                            let response = ui.selectable_label(is_selected, &folder.name);
                            if response.clicked() {
                                dispatch_folder_command(FolderCommand::SelectFolder {
                                    id: folder.id,
                                });
                            }
                        });
                    }
                }
            });
        });
}
