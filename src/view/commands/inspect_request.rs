use std::io::stdout;

use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::{CliWriterRepository, CrosstermCliWriter};
use crate::view::style::{Color, StyledStr};

pub struct InspectRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub writer: Writer,
}

impl InspectRequestExecutor<CrosstermCliWriter> {
    pub fn new(request_name: String) -> Self {
        InspectRequestExecutor {
            request_name,
            writer: CrosstermCliWriter::from(Box::new(stdout())),
        }
    }
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for InspectRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer.print_lines_styled([[
            StyledStr::from(" Request data of "),
            StyledStr::from(&self.request_name).with_color_text(Color::Yellow),
        ]]);
        self.writer.print_lines([BREAK_LINE]);

        let request_data = provider.get_request_saved(self.request_name).await?;
        let output = serde_json::to_string_pretty(&request_data)?;

        self.writer.print_lines([output]);
        self.writer.print_lines([BREAK_LINE]);

        Ok(())
    }
}
