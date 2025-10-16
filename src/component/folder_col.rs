#[derive(serde::Deserialize, serde::Serialize)]
pub struct FolderPanel {
    // No local state - access global state directly for maximum efficiency
}

impl Default for FolderPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("Folders Panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=600.0)
            .show(ctx, |ui| {
                ui.label("Scripts Folders");
                ui.separator();

                // Add folder button
                ui.vertical_centered(|ui| {
                    let response = ui.button("add folder");
                    if response.clicked() {
                        let _ =
                            crate::EVENT_SENDER
                                .get()
                                .unwrap()
                                .send(crate::AppMessage::Command(crate::AppCommand::Folder(
                                    crate::FolderCommand::CreateFolderCommand {},
                                )));
                    }
                });

                ui.add_space(10.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    crate::with_folder_state(|state| {
                        if state.folder_list.is_empty() {
                            ui.label("No folders yet...");
                        } else {
                            for folder in &state.folder_list {
                                ui.label(&folder.name);
                            }
                        }
                    });
                });
            });
    }
}

// Keep the old function for backward compatibility
pub fn folder_col(ctx: &egui::Context) {
    let mut panel = FolderPanel::new();
    panel.show(ctx);
}
