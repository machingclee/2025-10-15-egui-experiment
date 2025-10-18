pub async fn load_folders() {
    let db = crate::db::get_db::get_db();
    let folders = db.scripts_folder().find_many(vec![]).exec().await;
    match folders {
        Ok(folders) => {
            let state = crate::get_folder_state_ref();
            *state.folder_list.write().unwrap() = std::sync::Arc::new(folders);
        }
        Err(e) => eprintln!("Failed to load folders: {:?}", e),
    }
}
