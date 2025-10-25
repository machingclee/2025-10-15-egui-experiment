use std::sync::Arc;

use crate::db::get_db::get_db;
use crate::db::repository::folder_repository::FolderRepository;
use crate::db::repository::script_repository::ScriptRepository;
use crate::with_folder_state_reducer;

#[derive(Debug)]
pub enum FolderEvent {
    FolderAdded { name: String, ordering: i32 },
    FolderSelected { folder_id: i32 },
    FolderDeleted { folder_id: i32 },
    ScriptAdded { folder_id: i32 },
    ScriptUpdated { script_id: i32 },
    FolderRenamed { folder_id: i32, new_name: String },
    ScriptDeleted { script_id: i32 },
    FoldersReordered { from_index: i32, to_index: i32 },
}

pub struct FolderEventHandler {
    folder_repository: Arc<FolderRepository>,
    script_repository: Arc<ScriptRepository>,
}

impl FolderEventHandler {
    pub fn new() -> Self {
        Self {
            folder_repository: Arc::new(FolderRepository::new()),
            script_repository: Arc::new(ScriptRepository::new()),
        }
    }

    pub fn handle(&self, event: FolderEvent) {
        let folder_repository = self.folder_repository.clone();
        let script_repository = self.script_repository.clone();
        match event {
            FolderEvent::FoldersReordered{from_index, to_index} => {
                with_folder_state_reducer(
                    |reducer| reducer.insert_folder_into_index(from_index as usize, to_index as usize)
                )
            }
            FolderEvent::FolderAdded { name, ordering } => {
                // fetch all folder and set it into the state
                println!(
                    "Folder added event received for folder: {}, now refetch all folders",
                    name
                );
                crate::spawn_task(async move {
                    match folder_repository.get_all_folders().await {
                        Ok(folders) => {
                            crate::with_folder_state_reducer(|r| r.set_folder_list(folders));
                        }
                        Err(e) => eprintln!("Failed to load folders: {:?}", e),
                    }
                });
            }
            FolderEvent::FolderSelected { folder_id: id } => {
                println!("Folder selected event received for folder id: {}", id);
                crate::spawn_task(async move {
                    // upsert app_state to set last_folder_id to be this id
                    crate::with_folder_state_reducer(|r| r.select_folder(id));
                    println!("Loading related scripts");
                    match folder_repository.get_app_state().await {
                        Ok(Some(app_state)) => {
                            let folder_id_opt = app_state.last_opened_folder_id;
                            crate::with_folder_state_reducer(|r| r.set_app_state(Some(app_state)));

                            if let Some(folder_id) = folder_id_opt {
                                match script_repository
                                    .get_scripts_with_relations_by_folder(folder_id)
                                    .await
                                {
                                    Ok(folder_scripts) => {
                                        println!(
                                            "Found {} scripts for folder {}",
                                            folder_scripts.len(),
                                            folder_id
                                        );
                                        crate::with_folder_state_reducer(|r| {
                                            r.set_scripts_of_selected_folder(folder_scripts)
                                        });
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to load scripts for folder: {:?}", e)
                                    }
                                }
                            } else {
                                println!("No folder currently selected");
                            }
                        }
                        Ok(None) => {
                            println!("No application state found");
                        }
                        Err(e) => eprintln!("Failed to load application state: {:?}", e),
                    }
                });
            }
            FolderEvent::FolderDeleted { folder_id } => {
                with_folder_state_reducer(|reducer| reducer.delete_folder(folder_id));
                println!("Folder deleted event received for folder id: {}", folder_id);
            }
            FolderEvent::ScriptAdded { folder_id } => {
                crate::spawn_task(async move {
                    // must be those scripts of folder with folder_id, need to left join rel table
                    match script_repository.get_scripts_by_folder(folder_id).await {
                        Ok(scripts) => {
                            crate::with_folder_state_reducer(|r| {
                                r.set_scripts_of_selected_folder(scripts)
                            });
                        }
                        Err(e) => eprintln!("Failed to load scripts: {:?}", e),
                    }
                });
            }
            FolderEvent::ScriptUpdated { script_id } => {
                println!("Script updated event received for script id: {}", script_id);
                crate::with_folder_state(|state| {
                    if let Some(folder_id) = *state.selected_folder_id.read().unwrap() {
                        crate::spawn_task(async move {
                            match script_repository.get_scripts_by_folder(folder_id).await {
                                Ok(scripts) => {
                                    crate::with_folder_state_reducer(|r| {
                                        r.set_scripts_of_selected_folder(scripts)
                                    });
                                }
                                Err(e) => eprintln!("Failed to reload scripts: {:?}", e),
                            }
                        });
                    }
                })
            }
            FolderEvent::FolderRenamed {
                folder_id,
                new_name,
            } => {
                crate::with_folder_state_reducer(|r| r.rename_folder(folder_id, &new_name));
                println!(
                    "Folder renamed event received for folder id: {}, new name: {}",
                    folder_id, new_name
                );
            }
            FolderEvent::ScriptDeleted { script_id } => {
                println!("Script deleted event received for script id: {}", script_id);
                // just remove the script from UI state
                crate::with_folder_state_reducer(|r| {
                    r.delete_script_from_selected_folder(script_id)
                });
            }
        };
    }
    
}
