use crate::state::folder_state::FoldersState;
use crate::{
    dispatch_folder_command, domain::folder::folder_command_handler::FolderCommand,
    get_folder_state_ref, with_folder_state,
};
use egui::Ui;
use std::sync::Arc;

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

            Self::header(ui);
            self.add_script_button(ui);
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
    }

    fn rename_script_window(&mut self, ui: &mut Ui, script_id: i32) {
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

    fn edit_script_window(&mut self, ui: &mut Ui, script_id: i32) {
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

    fn scripts_of_selected_folder(&mut self, ui: &mut Ui) {
        crate::with_folder_state(|state| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                with_scritps_from_selected_folder(
                    |scripts| {
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
                    },
                );
            });
        })
    }

    fn new_script_window(&mut self, ui: &mut Ui) {
        with_selected_folder(|selected_folder| {
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
                                folder_id: selected_folder.unwrap().id,
                                name: "New Script".into(),
                                command: self.adding_code.clone(),
                            });
                            self.adding_new_script = false;
                        }
                    });
                });
        })
    }

    fn add_script_button(&mut self, ui: &mut Ui) {
        with_selected_folder(|selected_folder| {
            ui.vertical_centered(|ui| {
                let response = ui.add_enabled(selected_folder.is_some(), |ui: &mut egui::Ui| {
                    ui.button("add script")
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
            ui.label(format!("Scripts ({})", display_name));
            ui.separator();
        });
    }
}

// Pure view function - only handles rendering

// Approach 2: Callback pattern - pass view logic to data access
fn with_selected_folder<F, R>(f: F) -> R
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

fn with_scritps_from_selected_folder<F, R>(f: F) -> R
where
    F: FnOnce(Arc<Vec<crate::prisma::shell_script::Data>>) -> R,
{
    with_folder_state(|state| {
        let scripts = state.scripts_of_selected_folder.read().unwrap();
        f(scripts.clone())
    })
}
