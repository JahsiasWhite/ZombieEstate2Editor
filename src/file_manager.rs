use anyhow::Result;
use chrono::Local;
use notify::{Watcher, RecursiveMode};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

pub struct FileManager {
    game_path: PathBuf,
    backup_path: PathBuf,
    valid_extensions: Vec<String>,
}

impl FileManager {
    pub fn new(game_path: &Path) -> Self {
        let backup_path = PathBuf::from("backups");
        let valid_extensions = vec![
            "xml".to_string(),
            "xnb".to_string(),
            "chr".to_string(),
            "bul".to_string(),
        ];

        Self {
            game_path: game_path.to_path_buf(),
            backup_path,
            valid_extensions,
        }
    }

    pub fn get_backup_path(&self) -> &PathBuf {
        &self.backup_path
    }

    pub fn get_category_files(&self, category: &str) -> Result<Vec<PathBuf>> {
        let category_path = self.game_path.join(category);
        let mut files = Vec::new();
        
        if category_path.exists() {
            for entry in WalkDir::new(category_path) {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if self.valid_extensions.contains(&ext.to_string_lossy().to_string()) {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }

    pub fn create_new_character(&self, category: &str) -> Result<PathBuf> {
        let category_path = self.game_path.join(category);
        let default_file = category_path.join("DEFAULT.chr");
        let new_file = category_path.join("NEW.chr");

        if default_file.exists() {
            fs::copy(default_file, &new_file)?;
            Ok(new_file)
        } else {
            Err(anyhow::anyhow!("DEFAULT.chr not found in category path"))
        }
    }

}