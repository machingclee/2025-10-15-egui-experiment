use std::sync::Arc;

use crate::component::folder_col::FolderColumn;
use crate::component::{scripts_col::ScriptsColumn, top_menu::top_menu};
use crate::db::get_db::get_db;
use crate::dispatch_folder_command;
use crate::domain::folder::folder_command_handler::{FolderCommand, FolderCommandHandler};
use crate::domain::folder::folder_event_handler::{FolderEvent, FolderEventHandler};

pub struct App {
    folder_col: FolderColumn,
    scripts_col: ScriptsColumn,
    folder_command_handler: FolderCommandHandler,
    folder_event_handler: FolderEventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            folder_col: FolderColumn::new(),
            scripts_col: ScriptsColumn::new(),
            folder_command_handler: FolderCommandHandler::new(),
            folder_event_handler: FolderEventHandler::new(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::setup_custom_fonts(&cc.egui_ctx);

        // load the initial state from db:
        crate::spawn_task(async move {
            let db = get_db();
            match db.application_state().find_first(vec![]).exec().await {
                Ok(app_state) => {
                    crate::with_folder_state(|state| {
                        *state.app_state.write().unwrap() = Arc::new(app_state.clone());
                    });
                    if let Some(app_state_inner) = app_state {
                        if let Some(folder_id) = app_state_inner.last_opened_folder_id {
                            dispatch_folder_command(FolderCommand::SelectFolder {
                                folder_id: folder_id,
                            });
                        };
                    }
                }
                Err(e) => eprintln!("Failed to load application state: {:?}", e),
            }

            let folders = db.scripts_folder().find_many(vec![]).exec().await;
            match folders {
                Ok(folders) => {
                    crate::with_folder_state(|state| {
                        *state.folder_list.write().unwrap() = Arc::new(folders);
                    });
                }
                Err(e) => eprintln!("Failed to load folders: {:?}", e),
            }
        });

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set smaller heading font globally
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );
        ctx.set_style(style);
        while let Ok(message) = crate::EVENT_RECEIVER.get().unwrap().try_recv() {
            match message {
                crate::AppMessage::Command(cmd) => match cmd {
                    crate::AppCommand::Folder(folder_cmd) => {
                        self.folder_command_handler.handle(folder_cmd);
                    }
                },
                crate::AppMessage::Event(evt) => match evt {
                    crate::AppEvent::Folder(event) => {
                        self.folder_event_handler.handle(event);
                    }
                },
            }
        }

        top_menu(ctx);
        self.folder_col.view(ctx);
        self.scripts_col.view(ctx);
    }
}
