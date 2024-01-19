use std::path::PathBuf;

use tokio::sync::oneshot;

use super::service::FileServiceInstance;
use crate::app::service_commands::Command;

pub type CommandFileService<Resp> = Command<FileServiceInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn get_or_create_data_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            if let Ok(path_file) = service.get_or_create_data_file(path) {
                tx.send(path_file).ok();
            }
            service
        })
        .with_response(rx)
    }

    pub fn get_or_create_config_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            if let Ok(path_file) = service.get_or_create_config_file(path) {
                tx.send(path_file).ok();
            }

            service
        })
        .with_response(rx)
    }

    pub fn get_or_create_temp_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            if let Ok(path_file) = service.get_or_create_temp_file(path) {
                tx.send(path_file).ok();
            }
            service
        })
        .with_response(rx)
    }

    pub fn find_all_data_files() -> CommandFileService<Vec<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            if let Ok(path_file) = service.find_all_data_files() {
                tx.send(path_file).ok();
            }
            service
        })
        .with_response(rx)
    }

    pub fn remove_file(path: PathBuf) -> CommandFileService<()> {
        Command::from(|service: FileServiceInstance| {
            service.remove_file(path).ok();
            service
        })
    }
}
