pub async fn load_folders() {
    let db = crate::db::get_db::get_db();
    let folders = db.scripts_folder().find_many(vec![]).exec().await;
    match folders {
        Ok(folders) => {
            crate::with_folder_state_mut(|state| {
                state.folder_list = folders;
            });
        }
        Err(e) => eprintln!("Failed to load folders: {:?}", e),
    }
}
