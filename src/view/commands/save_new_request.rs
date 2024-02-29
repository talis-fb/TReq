use std::io::{empty, stderr};

use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::RequestData;
use crate::view::input::cli_input::ViewOptions;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::{CliWriterRepository, CrosstermCliWriter};
use crate::view::style::{Color, StyledStr};

pub struct SaveNewRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub request_data: RequestData,
    pub writer: Writer,
}

impl SaveNewRequestExecutor<CrosstermCliWriter> {
    pub fn new(
        request_name: String,
        request_data: RequestData,
        view_options: &ViewOptions,
    ) -> Self {
        if view_options.suppress_output {
            SaveNewRequestExecutor {
                request_name,
                request_data,
                writer: CrosstermCliWriter::from(Box::new(empty())),
            }
        } else {
            SaveNewRequestExecutor {
                request_name,
                request_data,
                writer: CrosstermCliWriter::from(Box::new(stderr())),
            }
        }
    }
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for SaveNewRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer
            .print_lines_styled([[StyledStr::from(" Saving").with_color_text(Color::Yellow)]]);
        self.writer.print_lines_styled([[
            StyledStr::from(" -> "),
            StyledStr::from(&self.request_name).with_color_text(Color::Blue),
        ]]);

        provider
            .save_request_datas_as(self.request_name, self.request_data)
            .await?;

        Ok(())
    }
}
