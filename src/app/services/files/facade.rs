use std::path::PathBuf;

pub trait FileServiceFacade: Send {
    fn get_or_create_config_file(&self, path: String) -> Result<PathBuf, String>;
    fn get_or_create_data_file(&self, path: String) -> Result<PathBuf, String>;
    fn get_or_create_temp_file(&self, path: String) -> Result<PathBuf, String>;
    fn remove_file(&self, path: PathBuf) -> Result<(), String>;
}
