use std::io::{empty, stdout};

use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::view::input::cli_input::ViewOptions;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::{CliWriterRepository, CrosstermCliWriter};
use crate::view::style::{Color, StyledStr};

pub struct RemoveRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub writer: Writer,
}

impl RemoveRequestExecutor<CrosstermCliWriter> {
    pub fn new(request_name: String, view_options: &ViewOptions) -> Self {
        if view_options.suppress_output {
            RemoveRequestExecutor {
                request_name,
                writer: CrosstermCliWriter::from(Box::new(empty())),
            }
        } else {
            RemoveRequestExecutor {
                request_name,
                writer: CrosstermCliWriter::from(Box::new(stdout())),
            }
        }
    }
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for RemoveRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer.print_lines_styled([[
            StyledStr::from(" Removing: ").with_color_text(Color::Red),
            StyledStr::from(&self.request_name).with_color_text(Color::Yellow),
        ]]);
        self.writer.print_lines([BREAK_LINE]);

        let _ = provider
            .get_request_saved(self.request_name.clone())
            .await?;
        provider.remove_request_saved(self.request_name).await?;

        self.writer.print_lines([" Ok "]);

        Ok(())
    }
}
