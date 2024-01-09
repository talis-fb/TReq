use std::io::{stderr, stdout};

use async_trait::async_trait;

use super::commands::CliCommand;
use super::output::writer::CrosstermCliWriter;
use crate::app::provider::Provider;

pub mod submit_request;

#[async_trait]
pub trait CliCommandExecutor {
    async fn execute(&mut self, provider: &mut dyn Provider) -> anyhow::Result<()>;
}

pub fn get_runner_of_command(command: CliCommand) -> impl CliCommandExecutor {
    let writer_stdout = CrosstermCliWriter {
        stdout: Box::new(stdout()),
    };
    let writer_stderr = CrosstermCliWriter {
        stdout: Box::new(stderr()),
    };

    match command {
        CliCommand::SubmitRequest { request } => submit_request::BasicRequestExecutor {
            request,
            writer_stdout,
            writer_stderr,
        },
        _ => todo!(),
    }
}
