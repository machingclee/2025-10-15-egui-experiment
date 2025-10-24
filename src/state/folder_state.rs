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

pub struct FolderReducer<'a> {
    pub state: &'a FoldersState,
}

impl<'a> FolderReducer<'a> {
    pub fn select_folder(&self, id: i32) {
        *self.state.selected_folder_id.write().unwrap() = Some(id);
    }

    pub fn delete_folder(&self, id: i32) {
        let mut folders = self.state.folder_list.write().unwrap();
        let updated_folders: Vec<_> = folders.iter().filter(|f| f.id != id).cloned().collect();
        *folders = Arc::new(updated_folders);
    }
    
    pub fn delete_script_from_selected_folder (&self, script_id: i32) {
        let mut scripts = self.state.scripts_of_selected_folder.write().unwrap();
        let updated_scripts: Vec<_> = scripts
            .iter()
            .filter(|s| s.id != script_id)
            .cloned()
            .collect();
        *scripts = Arc::new(updated_scripts);
    }

    pub fn rename_folder(&self, id: i32, new_name: &str) {
        let mut folders = self.state.folder_list.write().unwrap();
        let folders_vec = Arc::make_mut(&mut *folders);
        for folder in folders_vec.iter_mut() {
            if folder.id == id {
                folder.name = new_name.to_string();
                break;
            }
        }
    }

    pub fn set_folder_list(&self, folders: Vec<prisma::scripts_folder::Data>) {
        *self.state.folder_list.write().unwrap() = Arc::new(folders);
    }

    pub fn set_scripts_of_selected_folder(&self, scripts: Vec<prisma::shell_script::Data>) {
        *self.state.scripts_of_selected_folder.write().unwrap() = Arc::new(scripts);
    }

    pub fn set_app_state(&self, app_state: Option<prisma::application_state::Data>) {
        *self.state.app_state.write().unwrap() = Arc::new(app_state);
    }
}
