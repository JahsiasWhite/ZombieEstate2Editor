use anyhow::Result;
use chrono::Local;
use notify::{Watcher, RecursiveMode};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileManager {
    game_path: PathBuf,
    backup_path: PathBuf,
}

impl FileManager {
    pub fn new(game_path: &Path) -> Self {
        let backup_path = PathBuf::from("backups");
        Self {
            game_path: game_path.to_path_buf(),
            backup_path,
        }
    }

    pub fn get_backup_path(&self) -> &PathBuf {
        &self.backup_path
    }

    pub fn get_modifiable_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.game_path) {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    // Add .chr to the allowed extensions
                    if ext == "xml" || ext == "xnb" || ext == "chr" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
        Ok(files)
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
                        if ext == "xml" || ext == "xnb" || ext == "chr" {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }

    pub fn create_backup(&self) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let backup_dir = self.backup_path.join(timestamp.to_string());
        std::fs::create_dir_all(&backup_dir)?;

        for file in self.get_modifiable_files()? {
            let relative_path = file.strip_prefix(&self.game_path)?;
            let backup_file = backup_dir.join(relative_path);
            if let Some(parent) = backup_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&file, backup_file)?;
        }
        Ok(())
    }

    pub fn restore_backup(&self, backup_dir: &Path) -> Result<()> {
        for file in self.get_modifiable_files()? {
            let relative_path = file.strip_prefix(&self.game_path)?;
            let backup_file = backup_dir.join(relative_path);
            if backup_file.exists() {
                std::fs::copy(backup_file, file)?;
            }
        }
        Ok(())
    }
}