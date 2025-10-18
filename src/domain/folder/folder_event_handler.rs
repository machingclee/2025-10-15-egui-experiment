use std::sync::Arc;

use crate::db::get_db::get_db;

#[derive(Debug)]
pub enum FolderEvent {
    FolderAdded { name: String },
    FolderSelected { id: i32 },
    FolderDeleted { id: i32 },
    ScriptAdded { folder_id: i32 },
    ScriptUpdated { script_id: i32 },
    FolderRenamed { id: i32, new_name: String },
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
                crate::spawn_task(async move {
                    let db = get_db();
                    // upsert app_state to set last_folder_id to be this id
                    match db
                        .application_state()
                        .upsert(
                            crate::prisma::application_state::id::equals(1),
                            vec![
                                crate::prisma::application_state::last_opened_folder_id::set(Some(
                                    id,
                                )),
                            ],
                            vec![
                                crate::prisma::application_state::last_opened_folder_id::set(Some(
                                    id,
                                )),
                            ],
                        )
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            println!("Successfully updated last opened folder id to {}", id);
                        };
                        Err(e) => eprintln!("Failed to update last opened folder id: {:?}", e),
                    }

                    // Load scripts for the selected folder using left join approach
                    println!("Loading related scripts ...");
                    match db.application_state().find_first(vec![]).exec().await {
                        Ok(Some(app_state)) => {
                            if let Some(folder_id) = app_state.last_opened_folder_id {
                                // Get all scripts that belong to this folder via left join with relation table
                                match db
                                    .shell_script()
                                    .find_many(vec![])
                                    .with(
                                        crate::prisma::shell_script::rel_scriptsfolder_shellscript::fetch(vec![
                                            crate::prisma::rel_scriptsfolder_shellscript::scripts_folder_id::equals(folder_id)
                                        ])
                                    )
                                    .exec()
                                    .await
                                {
                                    Ok(scripts) => {
                                        // Filter scripts that actually belong to this folder
                                        let folder_scripts: Vec<_> = scripts.into_iter()
                                            .filter(|script| {
                                                script.rel_scriptsfolder_shellscript.as_ref()
                                                    .map(|rels| !rels.is_empty())
                                                    .unwrap_or(false)
                                            })
                                            .collect();

                                        println!("Found {} scripts for folder {}", folder_scripts.len(), folder_id);
                                        let state = crate::get_folder_state_ref();
                                        *state.scripts_of_selected_folder.write().unwrap() = Arc::new(folder_scripts);
                                    }
                                    Err(e) => eprintln!("Failed to load scripts for folder: {:?}", e),
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
            FolderEvent::FolderDeleted { id } => {
                println!("Folder deleted event received for folder id: {}", id);
            }
            FolderEvent::ScriptAdded { folder_id } => {
                crate::spawn_task(async move {
                    let db = get_db();
                    // must be those scripts of folder with folder_id, need to left join rel table
                    match db
                    .shell_script()
                    .find_many(vec![crate::prisma::shell_script::rel_scriptsfolder_shellscript::some(vec![crate::prisma::rel_scriptsfolder_shellscript::scripts_folder_id::equals(
                        folder_id,
                    )])])
                    .exec()
                    .await
                {
                    Ok(scripts) => {
                        let state = crate::get_folder_state_ref();
                        *state.scripts_of_selected_folder.write().unwrap() = Arc::new(scripts);
                    }
                    Err(e) => eprintln!("Failed to load scripts: {:?}", e),
                }
                })
            }
            FolderEvent::ScriptUpdated { script_id } => {
                println!("Script updated event received for script id: {}", script_id);
                // Reload scripts for the currently selected folder
                let state = crate::get_folder_state_ref();
                if let Some(folder_id) = *state.selected_folder_id.read().unwrap() {
                    crate::spawn_task(async move {
                        let db = get_db();
                        match db
                            .shell_script()
                            .find_many(vec![crate::prisma::shell_script::rel_scriptsfolder_shellscript::some(vec![crate::prisma::rel_scriptsfolder_shellscript::scripts_folder_id::equals(
                                folder_id,
                            )])])
                            .exec()
                            .await
                        {
                            Ok(scripts) => {
                                let state = crate::get_folder_state_ref();
                                *state.scripts_of_selected_folder.write().unwrap() = Arc::new(scripts);
                            }
                            Err(e) => eprintln!("Failed to reload scripts: {:?}", e),
                        }
                    });
                }
            }
            FolderEvent::FolderRenamed { id, new_name } => {
                println!(
                    "Folder renamed event received for folder id: {}, new name: {}",
                    id, new_name
                );
                let state = crate::get_folder_state_ref();
                let mut folders = state.folder_list.write().unwrap();
                let folders_vec = Arc::make_mut(&mut *folders);
                for folder in folders_vec.iter_mut() {
                    if folder.id == id {
                        folder.name = new_name.clone();
                        break;
                    }
                }
            }
        };
    }
}
