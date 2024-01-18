use std::path::PathBuf;

use tokio::sync::oneshot;

use super::service::FileServiceInstance;
use crate::utils::commands::{Command, ErrAtomic};

pub type CommandFileService<Resp> = Command<FileServiceInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn get_or_create_data_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let file = service.get_or_create_data_file(path);

            match file {
                Ok(path_file) => {
                    tx.send(path_file).ok();
                    Ok(service)
                }
                Err(error_message) => Err(ErrAtomic {
                    snapshot: service,
                    error_message,
                }),
            }
        })
        .with_response(rx)
    }

    pub fn get_or_create_config_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let file = service.get_or_create_config_file(path);

            match file {
                Ok(path_file) => {
                    tx.send(path_file).ok();
                    Ok(service)
                }
                Err(error_message) => Err(ErrAtomic {
                    snapshot: service,
                    error_message,
                }),
            }
        })
        .with_response(rx)
    }

    pub fn get_or_create_temp_file(path: String) -> CommandFileService<PathBuf> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let file = service.get_or_create_temp_file(path);

            match file {
                Ok(path_file) => {
                    tx.send(path_file).ok();
                    Ok(service)
                }
                Err(error_message) => Err(ErrAtomic {
                    snapshot: service,
                    error_message,
                }),
            }
        })
        .with_response(rx)
    }

    pub fn find_all_data_files() -> CommandFileService<Vec<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let files = service.find_all_data_files();

            match files {
                Ok(files) => {
                    tx.send(files).ok();
                    Ok(service)
                }
                Err(error_message) => Err(ErrAtomic {
                    snapshot: service,
                    error_message,
                }),
            }
        })
        .with_response(rx)
    }

    pub fn remove_file(path: PathBuf) -> CommandFileService<()> {
        Command::from(
            |service: FileServiceInstance| match service.remove_file(path) {
                Ok(_) => Ok(service),
                Err(error_message) => Err(ErrAtomic {
                    snapshot: service,
                    error_message,
                }),
            },
        )
    }
}
