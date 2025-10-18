use crate::component::folder_col::folder_col;
use crate::component::{scripts_col::scripts_col, top_menu::top_menu};
use crate::db::get_db::get_db;
use crate::domain::folder;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::setup_custom_fonts(&cc.egui_ctx);

        // load the initial state from db:
        crate::spawn_task(async move {
            let db = get_db();
            let folders = db.scripts_folder().find_many(vec![]).exec().await;
            match folders {
                Ok(folders) => {
                    let state = crate::get_folder_state_ref();
                    *state.folder_list.write().unwrap() = std::sync::Arc::new(folders);
                }
                Err(e) => eprintln!("Failed to load folders: {:?}", e),
            }
        });

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(message) = crate::EVENT_RECEIVER.get().unwrap().try_recv() {
            match message {
                crate::AppMessage::Command(cmd) => match cmd {
                    crate::AppCommand::Folder(folder_cmd) => {
                        folder::folder_command_handler::FolderCommandHandler::handle(folder_cmd);
                    }
                },
                crate::AppMessage::Event(evt) => match evt {
                    crate::AppEvent::Folder(event) => {
                        folder::folder_event_handler::FolderEventHandler::handle(event);
                    }
                },
            }
        }
        top_menu(ctx);
        folder_col(ctx);
        scripts_col(ctx);
    }
}
