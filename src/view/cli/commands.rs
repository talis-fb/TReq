#![allow(unused_variables)]

use std::io::{stderr, stdout};

use async_trait::async_trait;

use super::command_executors::{self, CommandExecutor};
use super::output::writer::CrosstermCliWriter;
use crate::app::backend::Backend;
use crate::app::services::request::entities::{OptionalRequestData, RequestData};
use crate::view::cli::command_executors::save_request_with_base_request::save_request_with_base_request_executor;

#[derive(Debug, PartialEq, Eq)]
pub enum CliCommand {
    SubmitRequest {
        request: RequestData,
    },

    SubmitSavedRequest {
        request_name: String,
        request_data: OptionalRequestData,
    },

    SaveRequest {
        request_name: String,
        request_data: OptionalRequestData,
        check_exists_before: bool,
    },
    SaveRequestWithBaseRequest {
        request_name: String,
        base_request_name: String,
        request_data: OptionalRequestData,
        check_exists_before: bool,
    },

    RemoveSavedRequest {
        request_name: String,
    },

    RenameSavedRequest {
        request_name: String,
        new_name: String,
    },
}

// ---------------
// Executors
// ---------------

#[async_trait]
pub trait CliCommandExecutor {
    async fn execute(&mut self, provider: &mut dyn Backend) -> anyhow::Result<()>;
}

pub fn get_executor_of_cli_command(command: CliCommand) -> CommandExecutor {
    let writer_stdout = CrosstermCliWriter::from(stdout());
    let writer_stderr = CrosstermCliWriter::from(stderr());

    use command_executors::save_request::save_request_executor;
    use command_executors::submit_request::basic_request_executor;
    use command_executors::submit_saved_request::submit_saved_request_executor;

    match command {
        CliCommand::SubmitRequest { request } => {
            basic_request_executor(request, writer_stdout, writer_stderr)
        }
        CliCommand::SubmitSavedRequest { request_name, request_data } => {
            submit_saved_request_executor(request_name, request_data, writer_stdout, writer_stderr)
        }

        CliCommand::SaveRequest {
            request_name,
            request_data,
            check_exists_before,
        } => save_request_executor(request_name, request_data, check_exists_before, writer_stdout, writer_stderr),
        CliCommand::SaveRequestWithBaseRequest {
            request_name,
            base_request_name,
            request_data,
            check_exists_before,
        } => save_request_with_base_request_executor(
            request_name,
            base_request_name,
            request_data,
            check_exists_before,
            writer_stdout,
            writer_stderr,
        ),

        CliCommand::RenameSavedRequest {
            request_name,
            new_name,
        } => todo!(),
        CliCommand::RemoveSavedRequest { request_name } => todo!(),
    }
}
