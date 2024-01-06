use std::path::PathBuf;

use tokio::sync::oneshot::{self, Receiver};

use super::service::FileServiceInstance;
use crate::utils::commands::{CommandClosureType, ErrAtomic};

pub type CommandFileService = CommandClosureType<FileServiceInstance>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn get_or_create_data_file(path: String) -> (CommandFileService, Receiver<PathBuf>) {
        let (tx, rx) = oneshot::channel();

        let command: CommandFileService = Box::new(|mut service| {
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
        });

        (command, rx)
    }

    pub fn get_or_create_config_file(path: String) -> (CommandFileService, Receiver<PathBuf>) {
        let (tx, rx) = oneshot::channel();

        let command: CommandFileService = Box::new(|mut service| {
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
        });

        (command, rx)
    }

    pub fn get_or_create_temp_file(path: String) -> (CommandFileService, Receiver<PathBuf>) {
        let (tx, rx) = oneshot::channel();

        let command: CommandFileService = Box::new(|mut service| {
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
        });

        (command, rx)
    }

    pub fn remove_file(path: PathBuf) -> CommandFileService {
        Box::new(|mut service| match service.remove_file(path) {
            Ok(_) => Ok(service),
            Err(error_message) => Err(ErrAtomic {
                snapshot: service,
                error_message,
            }),
        })
    }
}
