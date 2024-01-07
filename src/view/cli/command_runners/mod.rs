use std::io::{stderr, stdout};

use async_trait::async_trait;

use super::commands::CliCommand;
use super::output::writer::CrosstermCliWriter;
use crate::app::provider::Provider;

pub mod submit_request;

#[async_trait]
pub trait CliCommandRunner {
    async fn execute(&mut self, provider: impl Provider + Send) -> anyhow::Result<()>;
}

pub fn get_runner_of_command(command: CliCommand) -> impl CliCommandRunner {
    let writer_stdout = CrosstermCliWriter {
        stdout: Box::new(stdout()),
    };
    let writer_stderr = CrosstermCliWriter {
        stdout: Box::new(stderr()),
    };

    match command {
        CliCommand::BasicRequest { request } => {
            return submit_request::BasicRequestExecutor {
                request,
                writer_stdout,
                writer_stderr,
            }
        }
    }
}
