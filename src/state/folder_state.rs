use std::sync::LazyLock;

use egui::mutex::Mutex;

use crate::prisma;

#[derive(Default)]
pub struct FoldersState {
    pub selected_folder: Option<prisma::scripts_folder::Data>,
    pub folder_list: Vec<prisma::scripts_folder::Data>,
    pub display_scripts: Vec<prisma::shell_script::Data>,
}

pub static FOLDER_STATE: LazyLock<Mutex<FoldersState>> =
    LazyLock::new(|| Mutex::new(FoldersState::default()));
