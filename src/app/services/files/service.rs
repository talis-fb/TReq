use std::path::PathBuf;

use anyhow::Result;

use super::facade::FileServiceFacade;

pub type FileServiceInstance = Box<dyn FileServiceFacade>;

pub struct FileService {
    config_root_path: PathBuf,
    data_app_root_path: PathBuf,
    temp_root_path: PathBuf,
}

impl FileService {
    pub fn init(
        config_root_path: impl Into<PathBuf>,
        data_app_root_path: impl Into<PathBuf>,
        temp_root_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            config_root_path: config_root_path.into(),
            data_app_root_path: data_app_root_path.into(),
            temp_root_path: temp_root_path.into(),
        }
    }
}

impl FileServiceFacade for FileService {
    fn get_or_create_config_file(&self, path: String) -> Result<PathBuf> {
        let file_path = FileService::build_path(&self.config_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn get_or_create_data_file(&self, path: String) -> Result<PathBuf> {
        let file_path = FileService::build_path(&self.data_app_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn get_or_create_temp_file(&self, path: String) -> Result<PathBuf> {
        let file_path = FileService::build_path(&self.temp_root_path, path);
        FileService::create_file_if_not_exists(file_path)
    }

    fn find_all_data_files(&self) -> Result<Vec<PathBuf>> {
        let files = std::fs::read_dir(&self.data_app_root_path)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| !path.is_dir())
            .collect::<Vec<_>>();
        Ok(files)
    }

    fn remove_file(&self, path: PathBuf) -> Result<()> {
        Ok(std::fs::remove_file(path)?)
    }
}

impl FileService {
    fn build_path(base_path: &PathBuf, file_name: String) -> PathBuf {
        let mut path = base_path.clone();
        path.push(file_name);
        path
    }

    fn create_file_if_not_exists(path: PathBuf) -> Result<PathBuf> {
        if !path.exists() {
            std::fs::File::create(&path)?;
        }
        Ok(path)
    }
}
