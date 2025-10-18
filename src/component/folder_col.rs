use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;

pub struct FolderColumn {
    popup_open: bool,
}

impl FolderColumn {
    pub fn new() -> Self {
        Self { popup_open: false }
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
                            let mut folder_item = FolderItem::new();
                            folder_item.view(ui, folder, selected_id);
                        }
                    }
                });
            });
    }
}

struct FolderItem {
    popup_open: bool,
}

impl FolderItem {
    fn new() -> Self {
        Self { popup_open: false }
    }

    fn view(
        &mut self,
        ui: &mut egui::Ui,
        folder: &crate::prisma::scripts_folder::Data,
        selected_id: Option<i32>,
    ) {
        ui.horizontal(|ui| {
            let is_selected = selected_id == Some(folder.id);
            let response = egui::Frame::group(ui.style())
                .show(ui, |ui| {
                    let response = ui.selectable_label(is_selected, &folder.name);

                    if response.clicked() {
                        dispatch_folder_command(FolderCommand::SelectFolder { id: folder.id });
                    }
                })
                .inner;
        });
    }
}
