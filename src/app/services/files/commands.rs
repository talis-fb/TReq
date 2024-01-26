use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::oneshot;

use super::service::FileServiceInstance;
use crate::app::service_commands::Command;

pub mod requests;

pub type CommandFileService<Resp> = Command<FileServiceInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn get_or_create_data_file(path: String) -> CommandFileService<Result<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let resp = service.get_or_create_data_file(path);
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn get_or_create_config_file(path: String) -> CommandFileService<Result<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let resp = service.get_or_create_config_file(path);
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn get_or_create_temp_file(path: String) -> CommandFileService<Result<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let resp = service.get_or_create_temp_file(path);
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn find_all_data_files() -> CommandFileService<Result<Vec<PathBuf>>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let resp = service.find_all_data_files();
            tx.send(resp).ok();
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
