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

pub fn show_list_all_request_executor(
    mut writer: impl CliWriterRepository + 'static,
    // writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            let mut requests_names = provider.lock().await.find_all_request_name().await?;
            requests_names.sort();

            writer.print_lines([BREAK_LINE]);
            writer.print_lines_styled([[
                StyledStr::from(" Requests").with_color_text(Color::Yellow)
            ]]);

            for request_name in requests_names {
                writer.print_lines_styled([[
                    StyledStr::from(TAB_SPACE),
                    StyledStr::from(&request_name),
                ]]);
            }

            Ok(())
        })
    })
}
