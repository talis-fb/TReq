use async_trait::async_trait;

use super::CliCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::OptionalRequestData;
use crate::view::cli::output::utils::BREAK_LINE;
use crate::view::cli::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SaveRequestWithBaseRequestExecutor<W1, W2>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
{
    pub request_name: String,
    pub request_data: OptionalRequestData,
    pub check_exists_before: bool,
    pub writer_stdout: W1,
    pub writer_stderr: W2,
}

#[async_trait]
impl<W1, W2> CliCommand for SaveRequestWithBaseRequestExecutor<W1, W2>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
{
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer_stderr.print_lines([BREAK_LINE]);
        self.writer_stderr
            .print_lines_styled([[StyledStr::from(" Saving").with_color_text(Color::Yellow)]]);
        self.writer_stderr.print_lines_styled([[
            StyledStr::from(" -> "),
            StyledStr::from(&self.request_name).with_color_text(Color::Blue),
        ]]);

        if self.check_exists_before {
            provider
                .get_request_saved(self.request_name.clone())
                .await?;
        }

        provider
            .save_request_datas_as(self.request_name, self.request_data.to_request_data())
            .await?;

        Ok(())
    }
}
