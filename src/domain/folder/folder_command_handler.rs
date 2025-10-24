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
    DeleteScript {
        script_id: i32,
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

    pub fn handle(&self, wrapped: crate::WrappedFolderCommand) {
        let command = wrapped.command;
        let callback = wrapped.callback;

        match command {
            FolderCommand::CreateFolder {} => {
                let db = Arc::new(crate::db::get_db::get_db()).clone();
                let folder_repository = self.folder_repository.clone();
                crate::spawn_task(async move {
                    let total_num_folders = db.scripts_folder().count(vec![]).exec().await.unwrap();
                    let folder_name = format!("Folder {}", total_num_folders + 1);

                    match folder_repository.create_script_folder(&folder_name).await {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FolderAdded {
                                name: folder_name.clone(),
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
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    let result: Result<(), prisma_client_rust::QueryError> = db
                        .scripts_folder()
                        .delete(crate::prisma::scripts_folder::id::equals(folder_id))
                        .exec()
                        .await
                        .map(|_| ());

                    match result {
                        Ok(_) => {
                            println!(
                                "Folder with id {} and related data deleted successfully",
                                folder_id
                            );
                            crate::dispatch_folder_event(FolderEvent::FolderDeleted { folder_id });
                        }
                        Err(e) => eprintln!("Failed to delete folder: {:?}", e),
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
        }
    }
}
