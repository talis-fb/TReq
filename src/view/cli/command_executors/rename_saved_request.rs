#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::sync::Arc;

use tokio::sync::Mutex;

use super::CommandExecutor;
use crate::app::backend::Backend;
use crate::app::services::request::entities::OptionalRequestData;
use crate::view::cli::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

const BREAK_LINE: &str = "----------------------------------------";
const BREAK_LINE_WITH_GAP: &str = "  --------------------------------------";

const TAB_SPACE: &str = "  ";
const SINGLE_SPACE: &str = " ";

pub fn rename_request_executor(
    request_name: String,
    new_request_name: String,
    writer_stdout: impl CliWriterRepository + 'static,
    writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            todo!()
        })
    })
}
