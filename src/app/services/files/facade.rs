use std::path::PathBuf;

use anyhow::Result;

pub trait FileServiceFacade: Send {
    fn get_or_create_config_file(&self, path: String) -> Result<PathBuf>;
    fn get_or_create_data_file(&self, path: String) -> Result<PathBuf>;
    fn get_or_create_temp_file(&self, path: String) -> Result<PathBuf>;
    fn find_all_data_files(&self) -> Result<Vec<PathBuf>>;
    fn find_all_data_files_in_folders(&self, folders: &[&str]) -> Result<Vec<PathBuf>>;
    fn remove_file(&self, path: PathBuf) -> Result<()>;
    fn remove_data_file(&self, path: String) -> Result<()>;
    fn remove_temp_file(&self, path: String) -> Result<()>;
}
