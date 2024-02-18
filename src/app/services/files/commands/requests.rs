use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::oneshot;

use super::{CommandFileService, CommandsFactory};
use crate::app::service_commands::Command;
use crate::app::services::files::service::FileServiceInstance;

const REQUESTS_FOLDER: &str = "collection/";

impl CommandsFactory {
    pub fn get_or_create_file_of_saved_request(
        request_name: String,
    ) -> CommandFileService<Result<PathBuf>> {
        let (tx, rx) = oneshot::channel();

        Command::from(move |service: FileServiceInstance| {
            let resp = service.get_or_create_data_file(format!("{REQUESTS_FOLDER}/{request_name}"));
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn find_all_files_of_saved_requests() -> CommandFileService<Result<Vec<PathBuf>>> {
        let (tx, rx) = oneshot::channel();

        Command::from(|service: FileServiceInstance| {
            let resp = service.find_all_data_files_in_folders(&[REQUESTS_FOLDER]);
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn remove_file_saved_request(request_name: String) -> CommandFileService<Result<()>> {
        let (tx, rx) = oneshot::channel();

        Command::from(move |service: FileServiceInstance| {
            let resp = service.remove_data_file(format!("{REQUESTS_FOLDER}/{request_name}"));
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }

    pub fn rename_file_saved_request(
        request_name: String,
        new_name: String,
    ) -> CommandFileService<Result<()>> {
        let (tx, rx) = oneshot::channel();

        Command::from(move |service: FileServiceInstance| {
            let resp = service.rename_data_file(
                format!("{REQUESTS_FOLDER}{request_name}"),
                format!("{REQUESTS_FOLDER}{new_name}"),
            );
            tx.send(resp).ok();
            service
        })
        .with_response(rx)
    }
}
