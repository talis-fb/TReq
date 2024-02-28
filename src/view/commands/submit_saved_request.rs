use async_trait::async_trait;

use super::submit_request::BasicRequestExecutor;
use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SubmitSavedRequestExecutor<W1, W2, W3>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
    W3: CliWriterRepository,
{
    pub request_name: String,
    pub input_request_data: PartialRequestData,
    pub writer_metadata: W1,
    pub writer_response: W2,
    pub writer_stderr: W3,
}

#[async_trait]
impl<W1, W2, W3> ViewCommand for SubmitSavedRequestExecutor<W1, W2, W3>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
    W3: CliWriterRepository,
{
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        let request = provider
            .get_request_saved(self.request_name.clone())
            .await?;

        let request = request.merge(self.input_request_data);

        self.writer_metadata.print_lines([BREAK_LINE]);
        self.writer_metadata.print_lines_styled([[
            StyledStr::from(" Submit saved request").with_color_text(Color::Blue)
        ]]);
        self.writer_metadata.print_lines_styled([[
            StyledStr::from(" | -> "),
            StyledStr::from(&self.request_name),
        ]]);

        Box::new(BasicRequestExecutor {
            request,
            writer_metadata: self.writer_metadata,
            writer_response: self.writer_response,
            writer_stderr: self.writer_stderr,
        })
        .execute(provider)
        .await?;

        Ok(())
    }
}
