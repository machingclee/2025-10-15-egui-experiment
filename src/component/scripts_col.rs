use std::sync::Arc;

use crate::{dispatch_folder_command, domain::folder::folder_command_handler::FolderCommand};

pub struct ScriptsColumn {
    adding_new_script: bool,
    adding_code: String,
    code_lang: String,
    editing_script_id: Option<i32>,
    editing_command: String,
    renaming_script_id: Option<i32>,
    renaming_name: String,
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
        }
    }

    pub fn view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(-6.0);
            let state = crate::get_folder_state_ref();
            let selected_folder_id = with_selected_folder(state, |sf| sf.map(|f| f.id));

            render_scripts_header(
                ui,
                selected_folder_id
                    .map(|id| {
                        let folders = state.folder_list.read().unwrap();
                        folders.iter().find(|f| f.id == id).map(|f| f.name.clone())
                    })
                    .flatten(),
            );

            ui.vertical_centered(|ui| {
                let response = ui.add_enabled(selected_folder_id.is_some(), |ui: &mut egui::Ui| {
                    ui.button("add script")
                });
                if response.clicked() {
                    self.adding_new_script = true;
                }
            });

            if self.adding_new_script {
                let mut theme =
                    egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                ui.collapsing("Theme", |ui| {
                    ui.group(|ui| {
                        theme.ui(ui);
                        theme.clone().store_in_memory(ui.ctx());
                    });
                });

                let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        ui.style(),
                        &theme,
                        buf.as_str(),
                        &self.code_lang,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts_mut(|f| f.layout_job(layout_job))
                };

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
                                .layouter(&mut layouter),
                        );
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.adding_new_script = false;
                            }
                            if ui.button("Add").clicked() {
                                dispatch_folder_command(FolderCommand::AddScriptToFolder {
                                    folder_id: selected_folder_id.unwrap(),
                                    name: "New Script".into(),
                                    command: self.adding_code.clone(),
                                });
                                self.adding_new_script = false;
                            }
                        });
                    });
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                with_scritps_from_folder(state, |scripts| {
                    for script in scripts.iter() {
                        let frame =
                            egui::Frame::group(ui.style()).fill(ui.visuals().faint_bg_color);
                        frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("Name: {}", script.name));
                                if ui.button("Rename").clicked() {
                                    self.renaming_script_id = Some(script.id);
                                    self.renaming_name = script.name.clone();
                                }
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("Execute").clicked() {
                                            // Execute the script command
                                            crate::run_terminal_command(script.command.clone());
                                        }
                                        if ui.button("Edit").clicked() {
                                            self.editing_script_id = Some(script.id);
                                            self.editing_command = script.command.clone();
                                        }
                                        if ui.button("Copy").clicked() {
                                            ui.ctx().copy_text(script.command.clone());
                                        }
                                    },
                                );
                            });
                            ui.label("Command:");
                            egui::Frame::NONE
                                .fill(ui.visuals().code_bg_color)
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut script.command.clone())
                                            .font(egui::TextStyle::Monospace)
                                            .interactive(false)
                                            .frame(false)
                                            .desired_rows(5)
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                        });
                        ui.add_space(10.0);
                    }
                });
            });

            if let Some(script_id) = self.editing_script_id {
                egui::Window::new("Edit Script")
                    .collapsible(false)
                    .resizable(true)
                    .default_height(400.0)
                    .default_width(600.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.editing_command)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_rows(20)
                                .desired_width(580.0),
                        );
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.editing_script_id = None;
                            }
                            if ui.button("Save").clicked() {
                                dispatch_folder_command(FolderCommand::UpdateScriptToFolder {
                                    script_id,
                                    new_command: self.editing_command.clone(),
                                });
                                self.editing_script_id = None;
                            }
                        });
                    });
            }

            if let Some(script_id) = self.renaming_script_id {
                egui::Window::new("Rename Script")
                    .collapsible(false)
                    .resizable(true)
                    .default_height(200.0)
                    .default_width(400.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.label("New name:");
                        ui.text_edit_singleline(&mut self.renaming_name);
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.renaming_script_id = None;
                            }
                            if ui.button("Rename").clicked() {
                                dispatch_folder_command(FolderCommand::UpdateScriptNameToFolder {
                                    script_id,
                                    new_name: self.renaming_name.clone(),
                                });
                                self.renaming_script_id = None;
                            }
                        });
                    });
            }
        });
    }
}

// Pure view function - only handles rendering
fn render_scripts_header(ui: &mut egui::Ui, selected_folder_name: Option<String>) {
    let display_name = selected_folder_name
        .as_deref()
        .unwrap_or("No folder selected");
    ui.label(format!("Scripts ({})", display_name));
    ui.separator();
}

// Approach 2: Callback pattern - pass view logic to data access
fn with_selected_folder<F, R>(state: &'static crate::state::folder_state::FoldersState, f: F) -> R
where
    F: FnOnce(Option<&crate::prisma::scripts_folder::Data>) -> R,
{
    let folder_id = *state.selected_folder_id.read().unwrap();
    let folders = state.folder_list.read().unwrap();
    let selected_folder = folders.iter().find(|f| Some(f.id) == folder_id);
    f(selected_folder)
}

fn with_scritps_from_folder<F, R>(
    state: &'static crate::state::folder_state::FoldersState,
    f: F,
) -> R
where
    F: FnOnce(Arc<Vec<crate::prisma::shell_script::Data>>) -> R,
{
    let scripts = state.scripts_of_selected_folder.read().unwrap();
    f(scripts.clone())
}
