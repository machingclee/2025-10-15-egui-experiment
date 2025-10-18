use std::sync::{Arc, LazyLock, RwLock};

use crate::prisma;

#[derive(Default)]
pub struct FoldersState {
    pub selected_folder_id: RwLock<Option<i32>>,
    pub folder_list: RwLock<Arc<Vec<prisma::scripts_folder::Data>>>,
    pub display_scripts: RwLock<Vec<prisma::shell_script::Data>>,
}

pub static FOLDER_STATE: LazyLock<FoldersState> = LazyLock::new(|| FoldersState::default());
