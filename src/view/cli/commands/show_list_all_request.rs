use async_trait::async_trait;

use super::CliCommand;
use crate::app::backend::Backend;
use crate::view::cli::output::utils::{BREAK_LINE, TAB_SPACE};
use crate::view::cli::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct ShowListAllRequestExecutor<W1>
where
    W1: CliWriterRepository,
{
    pub writer: W1,
}

#[async_trait]
impl<W1> CliCommand for ShowListAllRequestExecutor<W1>
where
    W1: CliWriterRepository,
{
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        let mut requests_names = provider.find_all_request_name().await?;
        requests_names.sort();

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

// pub fn show_list_all_request_executor(
//     mut writer: impl CliWriterRepository + 'static,
//     // writer_stderr: impl CliWriterRepository + 'static,
// ) -> CommandExecutor {
//     Box::new(move |provider: Arc<Mutex<dyn Backend>>| {
//         tokio::spawn(async move {
//             let provider = provider.clone();
//
//             let mut requests_names = provider.lock().await.find_all_request_name().await?;
//             requests_names.sort();
//
//             writer.print_lines([BREAK_LINE]);
//             writer.print_lines_styled([[
//                 StyledStr::from(" Requests").with_color_text(Color::Yellow)
//             ]]);
//
//             for request_name in requests_names {
//                 writer.print_lines_styled([[
//                     StyledStr::from(TAB_SPACE),
//                     StyledStr::from(&request_name),
//                 ]]);
//             }
//
//             Ok(())
//         })
//     })
// }
