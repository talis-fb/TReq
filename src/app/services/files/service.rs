use std::io::Write;
use std::path::PathBuf;
use super::facade::Facade;

pub struct FileService {
    config_root_path: PathBuf,
    data_app_root_path: PathBuf,
    temp_root_path: PathBuf,
}

impl FileService {
    pub fn from(config_root_path: impl Into<PathBuf>, data_app_root_path: impl Into<PathBuf>) -> Self {
        Self {
            config_root_path: config_root_path.into(),
            data_app_root_path: data_app_root_path.into(),
            temp_root_path: data_app_root_path.into(),
        }
    }
}

impl FileService {
    fn build_path(base_path: &PathBuf, file_name: String) -> PathBuf {
        let mut path = base_path.clone();
        path.push(file_name);
        path
    }

    fn create_file_if_not_exists(path: PathBuf) -> Result<PathBuf, String> {
        if !path.exists() {
            std::fs::File::create(&path).map_err(|err| err.to_string())?;
        }
        Ok(path)
    }
}

impl Facade for FileService {
    fn get_or_create_config_file(&self, path: String) -> Result<PathBuf, String> {
        let file_path = FileService::build_path(&self.config_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn get_or_create_data_file(&self, path: String) -> Result<PathBuf, String> {
        let file_path = FileService::build_path(&self.data_app_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn get_or_create_temp_file(&self, path: String) -> Result<PathBuf, String> {
        let file_path = FileService::build_path(&self.temp_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn remove_file(&self, path: PathBuf) -> Result<(), String> {
        std::fs::remove_file(&path).map_err(|err| err.to_string())
    }

    fn write_to_file(&self, path: PathBuf, content: &str) -> Result<(), String> {
        std::fs::write(&path, content).map_err(|err| err.to_string())
    }

    fn append_to_file(&self, path: PathBuf, content: &str) -> Result<(), String> {
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .and_then(|mut file| file.write_all(content.as_bytes()))
            .map_err(|err| err.to_string())
    }}
