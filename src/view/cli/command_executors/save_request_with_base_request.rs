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

pub fn save_request_with_base_request_executor(
    request_name: String,
    base_request_name: String,
    request_data: OptionalRequestData,
    check_exists_before: bool,
    writer_stdout: impl CliWriterRepository + 'static,
    mut writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();

            let base_request_data = provider
                .lock()
                .await
                .get_request_saved(base_request_name.clone())
                .await?;

            writer_stderr.print_lines([BREAK_LINE]);
            writer_stderr
                .print_lines_styled([[StyledStr::from(" Saving").with_color_text(Color::Blue)]]);
            writer_stderr.print_lines_styled([[StyledStr::from(&format!(
                " | {} -> {}",
                &base_request_name, &request_name
            ))]]);

            if check_exists_before {
                provider
                    .lock()
                    .await
                    .get_request_saved(request_name.clone())
                    .await?;
            }

            let final_request_data = request_data.merge_with(base_request_data);

            provider
                .lock()
                .await
                .save_request_datas_as(request_name, final_request_data)
                .await?;

            Ok(())
        })
    })
}
