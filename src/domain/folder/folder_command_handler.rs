use crate::db::repository::folder_repository::{FolderOrderUpdate, FolderRepository};
use crate::db::repository::script_repository::ScriptRepository;
use crate::domain::folder::folder_event_handler::FolderEvent;
use prisma_client_rust::bigdecimal::ToPrimitive;
use std::sync::Arc;

#[derive(Debug)]
pub enum FolderCommand {
    CreateFolder {},
    SelectFolder {
        folder_id: i32,
    },
    DeleteFolder {
        folder_id: i32,
    },
    AddScriptToFolder {
        folder_id: i32,
        name: String,
        command: String,
    },
    UpdateScript {
        script_id: i32,
        new_command: String,
    },
    UpdateScriptName {
        script_id: i32,
        new_name: String,
    },
    RenameFolder {
        folder_id: i32,
        new_name: String,
    },
    DeleteScript {
        script_id: i32,
    },
    ReorderFolders {
        from_index: i32,
        to_index: i32,
    },
}

pub struct FolderCommandHandler {
    folder_repository: Arc<FolderRepository>,
    script_repository: Arc<ScriptRepository>,
}

impl FolderCommandHandler {
    pub fn new() -> Self {
        Self {
            folder_repository: Arc::new(FolderRepository::new()),
            script_repository: Arc::new(ScriptRepository::new()),
        }
    }

    pub fn handle(&self, wrapped: crate::WrappedFolderCommand) -> Result<(), ()> {
        let command = wrapped.command;
        let callback = wrapped.callback;

        match command {
            FolderCommand::CreateFolder {} => {
                let db = Arc::new(crate::db::get_db::get_db()).clone();
                let folder_repository = self.folder_repository.clone();
                crate::spawn_task(async move {
                    let total_num_folders = db.scripts_folder().count(vec![]).exec().await.unwrap();
                    let folder_name = "New Collection".to_string();

                    match folder_repository
                        .create_script_folder(&folder_name, total_num_folders.to_i32().unwrap())
                        .await
                    {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FolderAdded {
                                name: folder_name.clone(),
                                ordering: total_num_folders.to_i32().unwrap(),
                            });
                        }
                        Err(e) => eprintln!("Failed to add folder: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::SelectFolder { folder_id } => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    match db
                        .application_state()
                        .upsert(
                            crate::prisma::application_state::id::equals(1),
                            vec![
                                crate::prisma::application_state::last_opened_folder_id::set(Some(
                                    folder_id,
                                )),
                            ],
                            vec![
                                crate::prisma::application_state::last_opened_folder_id::set(Some(
                                    folder_id,
                                )),
                            ],
                        )
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FolderSelected { folder_id });
                            println!(
                                "Successfully updated last opened folder id to {}",
                                folder_id
                            );
                        }
                        Err(e) => eprintln!("Failed to update last opened folder id: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::DeleteFolder { folder_id } => {
                let folder_repository = self.folder_repository.clone();
                crate::spawn_task(async move {
                    match folder_repository.delete_script_folder(folder_id).await {
                        Ok(_) => {
                            println!(
                                "Folder with id {} and related data deleted successfully",
                                folder_id
                            );
                        }
                        Err(e) => eprintln!("Failed to delete folder: {:?}", e),
                    }

                    let all_folders = folder_repository.get_all_folders().await;
                    match all_folders {
                        Ok(folders) => {
                            let mut sorted_folders = folders;
                            sorted_folders.sort_by_key(|f| f.ordering);
                            for (index, folder) in sorted_folders.iter_mut().enumerate() {
                                folder.ordering = index as i32;
                            }
                            let order_update_param = sorted_folders
                                .iter()
                                .map(|f| FolderOrderUpdate {
                                    folder_id: f.id.to_i32().unwrap(),
                                    new_ordering: f.ordering,
                                })
                                .collect::<Vec<FolderOrderUpdate>>();

                            match folder_repository
                                .batch_order_update(order_update_param)
                                .await
                            {
                                Ok(_) => {
                                    crate::dispatch_folder_event(FolderEvent::FolderDeleted {
                                        folder_id,
                                    });
                                    println!(
                                        "Folder orderings updated successfully after deletion"
                                    );
                                }
                                Err(e) => eprintln!("Failed to update folder orderings: {:?}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to retrieve all folders: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::AddScriptToFolder {
                folder_id,
                name,
                command,
            } => {
                let script_repository = self.script_repository.clone();
                crate::spawn_task(async move {
                    match script_repository
                        .create_script(name.clone(), command.clone())
                        .await
                    {
                        Ok(created_script) => {
                            println!("created script: {:?}", created_script);
                            match script_repository
                                .create_script_relationship(folder_id, created_script.id)
                                .await
                            {
                                Ok(_) => {
                                    println!(
                                        "Script '{}' added to folder id {} successfully",
                                        name, folder_id
                                    );
                                    crate::dispatch_folder_event(FolderEvent::ScriptAdded {
                                        folder_id,
                                    });
                                }
                                Err(e) => eprintln!("Failed to create relationship: {:?}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to add script: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::RenameFolder {
                folder_id,
                new_name,
            } => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    match db
                        .scripts_folder()
                        .update_many(
                            vec![crate::prisma::scripts_folder::id::equals(folder_id)],
                            vec![crate::prisma::scripts_folder::name::set(new_name.clone())],
                        )
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            println!(
                                "Folder id {} renamed to '{}' successfully",
                                folder_id, new_name
                            );
                            crate::dispatch_folder_event(FolderEvent::FolderRenamed {
                                folder_id,
                                new_name,
                            });
                        }
                        Err(e) => eprintln!("Failed to rename folder: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::UpdateScript {
                script_id,
                new_command,
            } => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    match db
                        .shell_script()
                        .update_many(
                            vec![crate::prisma::shell_script::id::equals(script_id)],
                            vec![crate::prisma::shell_script::command::set(
                                new_command.clone(),
                            )],
                        )
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            println!("Script id {} updated successfully", script_id);
                            crate::dispatch_folder_event(FolderEvent::ScriptUpdated { script_id });
                        }
                        Err(e) => eprintln!("Failed to update script: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::UpdateScriptName {
                script_id,
                new_name,
            } => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    match db
                        .shell_script()
                        .update_many(
                            vec![crate::prisma::shell_script::id::equals(script_id)],
                            vec![crate::prisma::shell_script::name::set(new_name.clone())],
                        )
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            println!(
                                "Script id {} renamed to '{}' successfully",
                                script_id, new_name
                            );
                            crate::dispatch_folder_event(FolderEvent::ScriptUpdated { script_id });
                        }
                        Err(e) => eprintln!("Failed to rename script: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::DeleteScript { script_id } => {
                let script_repository = self.script_repository.clone();
                crate::spawn_task(async move {
                    match script_repository.delete_script(script_id).await {
                        Ok(_) => {
                            println!("Script id {} deleted successfully", script_id);
                            crate::dispatch_folder_event(FolderEvent::ScriptDeleted { script_id });
                        }
                        Err(e) => eprintln!("Failed to delete script: {:?}", e),
                    }

                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
            FolderCommand::ReorderFolders {
                from_index: from_index,
                to_index: to_index,
            } => {
                let folder_repository = self.folder_repository.clone();
                crate::spawn_task(async move {
                    let moving_downwards = from_index < to_index;
                    let index_bound =
                        (folder_repository.get_all_folders().await.unwrap().len() - 1).max(0);

                    match folder_repository
                        .reorder_folders(
                            from_index.to_usize().unwrap(),
                            to_index.to_usize().unwrap(),
                        )
                        .await
                    {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FoldersReordered {
                                from_index,
                                to_index: to_index.to_i32().unwrap(),
                            });
                        }
                        Err(e) => eprintln!("Failed to reorder folders: {:?}", e),
                    }
                    if let Some(cb) = callback {
                        let _ = crate::EVENT_SENDER
                            .get()
                            .unwrap()
                            .send(crate::AppMessage::Callback(cb));
                    }
                });
            }
        }
        Ok(())
    }
}
