use std::io::stdout;

use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::view::output::utils::{BREAK_LINE, TAB_SPACE};
use crate::view::output::writer::{CliWriterRepository, CrosstermCliWriter};
use crate::view::style::{Color, StyledStr};

pub struct ShowListAllRequestExecutor<Writer: CliWriterRepository> {
    pub writer: Writer,
}

impl ShowListAllRequestExecutor<CrosstermCliWriter> {
    pub fn new() -> Self {
        ShowListAllRequestExecutor {
            writer: CrosstermCliWriter::from(Box::new(stdout())),
        }
    }
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for ShowListAllRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        let mut requests_names = provider.find_all_request_name().await?;
        requests_names.sort();

        if requests_names.is_empty() {
            self.writer.print_lines([BREAK_LINE]);
            self.writer
                .print_lines_styled([[StyledStr::from(" No requests found")]]);
            self.writer.print_lines([BREAK_LINE]);
            return Ok(());
        }

        self.writer.print_lines([BREAK_LINE]);
        self.writer
            .print_lines_styled([[StyledStr::from(" Requests").with_color_text(Color::Yellow)]]);

        for request_name in requests_names {
            self.writer
                .print_lines_styled([[StyledStr::from(TAB_SPACE), StyledStr::from(&request_name)]]);
        }

        Ok(())
    }
}
