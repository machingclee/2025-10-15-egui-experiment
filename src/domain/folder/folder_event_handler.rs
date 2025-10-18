use std::sync::Arc;

use crate::db::get_db::get_db;

#[derive(Debug)]
pub enum FolderEvent {
    FolderAdded { name: String },
    FolderSelected { id: i32 },
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FolderEventHandler;

impl FolderEventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(event: FolderEvent) {
        match event {
            FolderEvent::FolderAdded { name } => {
                // fetch all folder and set it into the state
                println!(
                    "Folder added event received for folder: {}, now refetch all folders",
                    name
                );
                crate::spawn_task(async move {
                    let db = get_db();
                    match db.scripts_folder().find_many(vec![]).exec().await {
                        Ok(folders) => {
                            let state = crate::get_folder_state_ref();
                            *state.folder_list.write().unwrap() = Arc::new(folders);
                        }
                        Err(e) => eprintln!("Failed to load folders: {:?}", e),
                    }
                });
            }
            FolderEvent::FolderSelected { id } => {
                println!("Folder selected event received for folder id: {}", id);
            }
        };
    }
}
