use std::io::{stderr, stdout};

use async_trait::async_trait;

use super::command_executors::{self, CommandExecutor};
use super::output::writer::CrosstermCliWriter;
use crate::app::backend::Backend;
use crate::app::services::request::entities::{OptionalRequestData, RequestData};

#[derive(Debug, PartialEq, Eq)]
pub enum CliCommand {
    SubmitRequest {
        request: RequestData,
    },

    SubmitSavedRequest {
        request_name: String,
    },
    SubmitSavedRequestWithAdditionalData {
        request_name: String,
        request_data: OptionalRequestData,
    },

    SaveRequest {
        request_name: String,
        request: OptionalRequestData,
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

    use command_executors::submit_request::basic_request_executor;
    use command_executors::submit_saved_request::submit_saved_request_executor;


    match command {
        CliCommand::SubmitRequest { request } => basic_request_executor(
            request,
            writer_stdout,
            writer_stderr,
        ),
        CliCommand::SubmitSavedRequest { request_name } => submit_saved_request_executor(
            request_name,
            writer_stdout,
            writer_stderr,
        ),
        _ => todo!(),
    }
}
