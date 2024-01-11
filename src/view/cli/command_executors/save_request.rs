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

pub fn save_request_executor(
    request_name: String,
    request_data: OptionalRequestData,
    writer_stdout: impl CliWriterRepository + 'static,
    mut writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            provider
                .lock()
                .await
                .save_request_datas_as(request_name.clone(), request_data.to_request_data())
                .await?;

            writer_stderr.print_lines([BREAK_LINE]);
            writer_stderr.print_lines_styled([[
                StyledStr::from(" Saving ").with_color_text(Color::Yellow),
                StyledStr::from(" -> "),
                StyledStr::from(&request_name).with_color_text(Color::Blue),
            ]]);

            Ok(())
        })
    })
}
