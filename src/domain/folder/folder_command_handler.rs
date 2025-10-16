use crate::domain::folder::folder_event_handler::FolderEvent;

#[derive(Debug)]
pub enum FolderCommand {
    CreateFolderCommand {},
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
                            crate::send_folder_event(FolderEvent::FolderAdded {
                                name: folder_name,
                            });
                        }
                        Err(e) => eprintln!("Failed to add folder: {:?}", e),
                    }
                });
            }
        }
    }
}
