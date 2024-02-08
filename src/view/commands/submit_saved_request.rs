use async_trait::async_trait;

use super::submit_request::BasicRequestExecutor;
use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::OptionalRequestData;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SubmitSavedRequestExecutor<W1, W2>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
{
    pub request_name: String,
    pub request_data: OptionalRequestData,
    pub writer_stdout: W1,
    pub writer_stderr: W2,
}

#[async_trait]
impl<W1, W2> ViewCommand for SubmitSavedRequestExecutor<W1, W2>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
{
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        let request = provider
            .get_request_saved(self.request_name.clone())
            .await?;

        let request_data = self.request_data.merge_with(request);

        self.writer_stderr.print_lines([BREAK_LINE]);
        self.writer_stderr.print_lines_styled([[
            StyledStr::from(" Submit saved request").with_color_text(Color::Blue)
        ]]);
        self.writer_stderr.print_lines_styled([[
            StyledStr::from(" | -> "),
            StyledStr::from(&self.request_name),
        ]]);

        Box::new(BasicRequestExecutor {
            request: request_data,
            writer_stdout: self.writer_stdout,
            writer_stderr: self.writer_stderr,
        })
        .execute(provider)
        .await?;

        Ok(())
    }
}
