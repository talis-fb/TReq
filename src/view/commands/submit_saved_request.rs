use async_trait::async_trait;

use super::submit_request::BasicRequestExecutor;
use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SubmitSavedRequestExecutor<W1, W2>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
{
    pub request_name: String,
    pub input_request_data: PartialRequestData,
    pub print_body_only: bool,
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

        let request = request.merge(self.input_request_data);

        self.writer_stderr.print_lines([BREAK_LINE]);
        self.writer_stderr.print_lines_styled([[
            StyledStr::from(" Submit saved request").with_color_text(Color::Blue)
        ]]);
        self.writer_stderr.print_lines_styled([[
            StyledStr::from(" | -> "),
            StyledStr::from(&self.request_name),
        ]]);

        Box::new(BasicRequestExecutor {
            request,
            print_body_only: self.print_body_only,
            writer_stdout: self.writer_stdout,
            writer_stderr: self.writer_stderr,
        })
        .execute(provider)
        .await?;

        Ok(())
    }
}
