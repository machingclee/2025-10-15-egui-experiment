use crate::db::repository::folder_repository::FolderRepository;
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

    pub fn handle(&self, command: FolderCommand) {
        let folder_repository = self.folder_repository.clone();
        let script_repository = self.script_repository.clone();
        match command {
            FolderCommand::CreateFolder {} => {
                crate::spawn_task(async move {
                    let total_num_folders =
                        folder_repository.get_folder_count().await.to_i64().unwrap();
                    let folder_name = format!("Folder {}", total_num_folders + 1);

                    match folder_repository.create_script_folder(&folder_name).await {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FolderAdded {
                                name: folder_name.clone(),
                            });
                        }
                        Err(e) => eprintln!("Failed to add folder: {:?}", e),
                    }
                });
            }
            FolderCommand::SelectFolder { folder_id } => crate::spawn_task(async move {
                match folder_repository
                    .upsert_app_state_last_folder_id(folder_id)
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
            }),
            FolderCommand::DeleteFolder { folder_id } => {
                crate::spawn_task(async move {
                    // Manual cascading delete (not atomic - use transaction for production)
                    let result: Result<(), prisma_client_rust::QueryError> =
                        folder_repository.delete_script_folder(folder_id).await;

                    match result {
                        Ok(_) => {
                            println!(
                                "Folder with id {} and related data deleted successfully",
                                folder_id
                            );
                            // Dispatch event to refresh UI
                            crate::dispatch_folder_event(FolderEvent::FolderDeleted { folder_id });
                        }
                        Err(e) => eprintln!("Failed to delete folder: {:?}", e),
                    }
                });
            }
            FolderCommand::AddScriptToFolder {
                folder_id,
                name,
                command,
            } => {
                crate::spawn_task(async move {
                    match script_repository
                        .create_script(name.clone(), command.clone())
                        .await
                    {
                        Ok(created_script) => {
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
                });
            }
            FolderCommand::UpdateScript {
                script_id,
                new_command,
            } => {
                crate::spawn_task(async move {
                    match script_repository
                        .update_script_command(script_id, new_command.clone())
                        .await
                    {
                        Ok(_) => {
                            println!("Script id {} updated successfully", script_id);
                            // Dispatch event to refresh scripts for the folder
                            // Assuming we need to find the folder_id, but for simplicity, dispatch a general event
                            crate::dispatch_folder_event(FolderEvent::ScriptUpdated { script_id });
                        }
                        Err(e) => eprintln!("Failed to update script: {:?}", e),
                    }
                });
            }
            FolderCommand::UpdateScriptName {
                script_id,
                new_name,
            } => {
                crate::spawn_task(async move {
                    match script_repository
                        .update_script_name(script_id, new_name.clone())
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
                });
            }
        }
    }
}
