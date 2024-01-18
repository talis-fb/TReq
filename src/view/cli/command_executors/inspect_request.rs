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

pub fn inspect_request_executor(
    request_name: String,
    mut writer: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            writer.print_lines([BREAK_LINE]);
            writer.print_lines_styled([[ StyledStr::from(" Request data of "), StyledStr::from(&request_name).with_color_text(Color::Yellow) ]]);
            writer.print_lines([BREAK_LINE]);

            let request_data = provider.lock().await.get_request_saved(request_name).await?;
            let output = serde_json::to_string_pretty(&request_data)?;

            writer.print_lines([ output ]);
            writer.print_lines([BREAK_LINE]);

            Ok(())
        })
    })
}
