use std::sync::{Arc, LazyLock, RwLock};

use crate::prisma;

#[derive(Default)]
pub struct FoldersState {
    pub selected_folder_id: RwLock<Option<i32>>,
    pub app_state: RwLock<Arc<Option<prisma::application_state::Data>>>,
    pub folder_list: RwLock<Arc<Vec<prisma::scripts_folder::Data>>>,
    pub scripts_of_selected_folder: RwLock<Arc<Vec<prisma::shell_script::Data>>>,
    pub folder_to_delete: RwLock<Option<Arc<prisma::scripts_folder::Data>>>,
    pub folder_to_rename: RwLock<Option<Arc<prisma::scripts_folder::Data>>>,
    pub rename_text: RwLock<Option<String>>,
    pub script_to_edit: RwLock<Option<Arc<prisma::shell_script::Data>>>,
}

pub static FOLDER_STATE: LazyLock<FoldersState> = LazyLock::new(|| FoldersState::default());
