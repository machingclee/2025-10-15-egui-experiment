use crate::component::common::horizontal_filled_button::horizontal_filled_button;
use crate::with_folder_state;
use egui::Ui;
use std::sync::Arc;

pub struct ScriptsColumn {
    pub adding_new_script: bool,
    pub adding_code: String,
    pub code_lang: String,
    pub editing_script_id: Option<i32>,
    pub editing_command: String,
    pub renaming_script_id: Option<i32>,
    pub renaming_name: String,
    pub script_to_delete: Option<i32>,
}

impl ScriptsColumn {
    pub fn new() -> Self {
        Self {
            adding_new_script: false,
            adding_code: "# add your code here ...".into(),
            code_lang: "bash".into(),
            editing_script_id: None,
            editing_command: String::new(),
            renaming_script_id: None,
            renaming_name: String::new(),
            script_to_delete: None,
        }
    }

    pub fn view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(-6.0);
            ui.add_space(10.0);
            Self::header(ui);
            ui.add_space(10.0);
            self.add_script_button(ui);
            ui.add_space(10.0);
            self.scripts_of_selected_folder(ui);

            // Pop-up windows
            self.conditionally_popup_windows(ui);
        });
    }

    fn conditionally_popup_windows(&mut self, ui: &mut Ui) {
        if self.adding_new_script {
            self.new_script_window(ui);
        }
        if let Some(editing_script_id) = self.editing_script_id {
            self.edit_script_window(ui, editing_script_id);
        }
        if let Some(renaming_script_id) = self.renaming_script_id {
            self.rename_script_window(ui, renaming_script_id);
        }
        if let Some(script_id) = self.script_to_delete {
            self.confirm_delete_script_window(ui, script_id);
        }
    }

    fn add_script_button(&mut self, ui: &mut Ui) {
        with_selected_folder(|selected_folder| {
            ui.vertical_centered(|ui| {
                let response = ui.add_enabled(selected_folder.is_some(), |ui: &mut egui::Ui| {
                    horizontal_filled_button(
                        ui,
                        ui.available_width(),
                        26.0,
                        "Add Script".into(),
                        "add_script_button".into(),
                    )
                });
                if response.clicked() {
                    self.adding_new_script = true;
                }
            });
        })
    }

    fn header(ui: &mut Ui) {
        with_selected_folder(|selected_folder| {
            let selected_folder_name = selected_folder.map(|f| f.name.clone());
            let display_name = selected_folder_name
                .as_deref()
                .unwrap_or("No folder selected");
            let icon = if selected_folder.is_some() {
                "📝" // Script icon when folder is selected
            } else {
                "⚠️" // Empty mailbox when no folder selected
            };
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(icon).font(egui::FontId::proportional(30.0)));
                ui.label(
                    egui::RichText::new(display_name)
                        .strong()
                        .font(egui::FontId::proportional(16.0)),
                );
            });
            ui.separator();
        });
    }
}

// Pure view function - only handles rendering

// Approach 2: Callback pattern - pass view logic to data access
pub fn with_selected_folder<F, R>(f: F) -> R
where
    F: FnOnce(Option<&crate::prisma::scripts_folder::Data>) -> R,
{
    crate::with_folder_state(|state| {
        let folder_id = *state.selected_folder_id.read().unwrap();
        let folders = state.folder_list.read().unwrap();
        let selected_folder = folders.iter().find(|f| Some(f.id) == folder_id);
        f(selected_folder)
    })
}

pub fn with_scritps_from_selected_folder<F, R>(f: F) -> R
where
    F: FnOnce(Arc<Vec<crate::prisma::shell_script::Data>>) -> R,
{
    with_folder_state(|state| {
        let scripts = state.scripts_of_selected_folder.read().unwrap();
        f(scripts.clone())
    })
}
