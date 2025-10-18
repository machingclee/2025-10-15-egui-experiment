use crate::domain::folder::folder_event_handler::FolderEvent;

#[derive(Debug)]
pub enum FolderCommand {
    CreateFolderCommand {},
    SelectFolder { id: i32 },
    DeleteFolder { id: i32 },
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FolderCommandHandler;

impl FolderCommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(command: FolderCommand) {
        match command {
            FolderCommand::CreateFolderCommand {} => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();
                    let total_num_folders = db.scripts_folder().count(vec![]).exec().await.unwrap();
                    let folder_name = format!("Folder {}", total_num_folders + 1);
                    match db
                        .scripts_folder()
                        .create(folder_name.clone(), 0, vec![])
                        .exec()
                        .await
                    {
                        Ok(_) => {
                            crate::dispatch_folder_event(FolderEvent::FolderAdded {
                                name: folder_name,
                            });
                        }
                        Err(e) => eprintln!("Failed to add folder: {:?}", e),
                    }
                });
            }
            FolderCommand::SelectFolder { id } => {
                let state = crate::get_folder_state_ref();
                *state.selected_folder_id.write().unwrap() = Some(id);
                crate::dispatch_folder_event(FolderEvent::FolderSelected { id });
            }
            FolderCommand::DeleteFolder { id } => {
                crate::spawn_task(async move {
                    let db = crate::db::get_db::get_db();

                    // Manual cascading delete (not atomic - use transaction for production)
                    let result: Result<(), ::prisma_client_rust::QueryError> = (async {
                        // 1. Find all scripts related to this folder
                        let related_scripts = db
                            .rel_scriptsfolder_shellscript()
                            .find_many(vec![crate::prisma::rel_scriptsfolder_shellscript::scripts_folder_id::equals(id)])
                            .with(crate::prisma::rel_scriptsfolder_shellscript::shell_script::fetch())
                            .exec()
                            .await?;

                        // 2. Delete relationship records first
                        db.rel_scriptsfolder_shellscript()
                            .delete_many(vec![crate::prisma::rel_scriptsfolder_shellscript::scripts_folder_id::equals(id)])
                            .exec()
                            .await?;

                        // 3. Delete scripts that are only used by this folder
                        for relation in related_scripts {
                            if let Some(script) = relation.shell_script {
                                // Check if this script is used by other folders
                                let other_relations = db
                                    .rel_scriptsfolder_shellscript()
                                    .find_many(vec![crate::prisma::rel_scriptsfolder_shellscript::shell_script_id::equals(script.id)])
                                    .exec()
                                    .await?;

                                // Only delete if no other folders reference this script
                                if other_relations.is_empty() {
                                    db.shell_script()
                                        .delete_many(vec![crate::prisma::shell_script::id::equals(script.id)])
                                        .exec()
                                        .await?;
                                }
                            }
                        }

                        // 4. Finally delete the folder
                        db.scripts_folder()
                            .delete_many(vec![crate::prisma::scripts_folder::id::equals(id)])
                            .exec()
                            .await?;

                        Ok(())
                    }).await;

                    match result {
                        Ok(_) => {
                            println!(
                                "Folder with id {} and related data deleted successfully",
                                id
                            );
                            // Update state to remove the folder from the list
                            let state = crate::get_folder_state_ref();
                            let mut folder_list = state.folder_list.write().unwrap();
                            let updated_folders: Vec<_> = folder_list
                                .iter()
                                .filter(|folder| folder.id != id)
                                .cloned()
                                .collect();
                            *folder_list = std::sync::Arc::new(updated_folders);
                            // Dispatch event to refresh UI
                            crate::dispatch_folder_event(FolderEvent::FolderDeleted { id });
                        }
                        Err(e) => eprintln!("Failed to delete folder: {:?}", e),
                    }
                });
            }
        }
    }
}
