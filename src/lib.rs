use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    id: String,
    name: String,
    folder_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder {
    id: String,
    name: String,
    parent_id: Option<String>,
}

pub struct FileSystem {
    files: HashMap<String, File>,
    folders: HashMap<String, Folder>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            files: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    pub fn create_folder(&mut self, name: String, parent_folder_id: Option<String>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.folders.insert(id.clone(), Folder {
            id: id.clone(),
            name,
            parent_id: parent_folder_id,
        });
        id
    }

    pub fn rename_folder(&mut self, folder_id: String, name: String) -> bool {
        if let Some(folder) = self.folders.get_mut(&folder_id) {
            folder.name = name;
            true
        } else {
            false
        }
    }

    pub fn move_folder(&mut self, source_id: String, target_id: String) -> bool {
        if let Some(folder) = self.folders.get_mut(&source_id) {
            folder.parent_id = Some(target_id);
            true
        } else {
            false
        }
    }

    pub fn search_folders(&self, name: String, root_id: Option<String>) -> Vec<&Folder> {
        let results: Vec<&Folder> = self.folders
            .values()
            .filter(|folder| {
                folder.name.contains(&name) && 
                folder.parent_id == root_id
            })
            .collect();
        
        results
    }

    pub fn create_file(&mut self, id: String, name: String, folder_id: String) -> bool {
        if self.folders.contains_key(&folder_id) {
            self.files.insert(id.clone(), File {
                id,
                name,
                folder_id,
            });
            true
        } else {
            false
        }
    }

    pub fn move_file(&mut self, file_id: String, folder_id: String) -> bool {
        if let Some(file) = self.files.get_mut(&file_id) {
            if self.folders.contains_key(&folder_id) {
                file.folder_id = folder_id;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn search_files(&self, name: String, root_id: Option<String>) -> Vec<&File> {
        let results: Vec<&File> = self.files
            .values()
            .filter(|file| {
                file.name.contains(&name) && 
                (root_id.is_none() || file.folder_id == root_id.as_ref().unwrap_or(&"".to_string()).to_string())
            })
            .collect();
        

        results
    }
} 