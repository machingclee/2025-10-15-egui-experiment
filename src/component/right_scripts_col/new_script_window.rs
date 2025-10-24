use crate::component::right_scripts_col::scripts_col::ScriptsColumn;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::scripts_folder::Data;
use eframe::epaint::Galley;
use egui::Ui;
use std::sync::Arc;

impl ScriptsColumn {
    pub fn new_script_window(&mut self, ui: &mut Ui) {
        crate::component::right_scripts_col::scripts_col::with_selected_folder(|selected_folder| {
            let mut theme =
                egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            ui.collapsing("Theme", |ui| {
                ui.group(|ui| {
                    theme.ui(ui);
                    theme.clone().store_in_memory(ui.ctx());
                });
            });

            let code_lang = self.code_lang.clone();
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &theme,
                    buf.as_str(),
                    &code_lang,
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts_mut(|f| f.layout_job(layout_job))
            };

            self.launch_add_script_window(ui, selected_folder, &mut layouter);
        })
    }
    fn launch_add_script_window(
        &mut self,
        ui: &mut Ui,
        selected_folder: Option<&Data>,
        layouter: &mut impl FnMut(&egui::Ui, &dyn egui::TextBuffer, f32) -> Arc<Galley>,
    ) {
        egui::Window::new("Add Script")
            .collapsible(false)
            .resizable(true)
            .default_height(400.0)
            .default_width(600.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.adding_code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(20)
                        .desired_width(580.0)
                        .layouter(layouter),
                );
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.adding_new_script = false;
                    }
                    if ui.button("Add").clicked() {
                        dispatch_folder_command(FolderCommand::AddScriptToFolder {
                            folder_id: selected_folder.unwrap().id,
                            name: "New Script".into(),
                            command: self.adding_code.clone(),
                        });
                        self.adding_new_script = false;
                    }
                });
            });
    }
}
