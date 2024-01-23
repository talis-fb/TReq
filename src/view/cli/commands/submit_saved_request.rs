use async_trait::async_trait;

use super::submit_request::BasicRequestExecutor;
use super::CliCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::OptionalRequestData;
use crate::view::cli::output::utils::BREAK_LINE;
use crate::view::cli::output::writer::CliWriterRepository;
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
impl<W1, W2> CliCommand for SubmitSavedRequestExecutor<W1, W2>
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

// pub fn submit_saved_request_executor(
//     request_name: String,
//     optional_request_data: OptionalRequestData,
//     writer_stdout: impl CliWriterRepository + 'static,
//     mut writer_stderr: impl CliWriterRepository + 'static,
// ) -> CommandExecutor {
//     Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
//         tokio::spawn(async move {
//             let provider = provider.clone();
//
//             let request = provider
//                 .lock()
//                 .await
//                 .get_request_saved(request_name.clone())
//                 .await?;
//
//             let request_data = optional_request_data.merge_with(request);
//
//             writer_stderr.print_lines([BREAK_LINE]);
//             writer_stderr.print_lines_styled([[
//                 StyledStr::from(" Submit saved request").with_color_text(Color::Blue)
//             ]]);
//             writer_stderr
//                 .print_lines_styled([[StyledStr::from(" | -> "), StyledStr::from(&request_name)]]);
//
//             basic_request_executor(request_data, writer_stdout, writer_stderr)(provider).await??;
//
//             Ok(())
//         })
//     })
// }
