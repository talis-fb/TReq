#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::sync::Arc;

use tokio::sync::Mutex;

use super::submit_request::basic_request_executor;
use super::CommandExecutor;
use crate::app::backend::Backend;
use crate::view::cli::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

const BREAK_LINE: &str = "----------------------------------------";
const BREAK_LINE_WITH_GAP: &str = "  --------------------------------------";

const TAB_SPACE: &str = "  ";
const SINGLE_SPACE: &str = " ";

pub fn submit_saved_request_executor(
    request_name: String,
    writer_stdout: impl CliWriterRepository + 'static,
    mut writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            let request = provider
                .lock()
                .await
                .get_request_saved(request_name.clone())
                .await?;

            writer_stderr.print_lines([BREAK_LINE]);
            writer_stderr.print_lines_styled([[
                StyledStr::from(" Submit saved request").with_color_text(Color::Blue)
            ]]);
            writer_stderr
                .print_lines_styled([[StyledStr::from(" | -> "), StyledStr::from(&request_name)]]);

            basic_request_executor(request, writer_stdout, writer_stderr)(provider).await??;

            Ok(())
        })
    })
}
