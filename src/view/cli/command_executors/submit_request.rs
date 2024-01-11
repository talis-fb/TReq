use std::sync::Arc;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use super::CommandExecutor;
use crate::app::backend::Backend;
use crate::app::services::request::entities::RequestData;
use crate::app::services::web_client::entities::get_status_code_message;
use crate::utils::observable::chain_listener_to_receiver;
use crate::view::cli::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr, TextStyle};

const BREAK_LINE: &str = "----------------------------------------";
const BREAK_LINE_WITH_GAP: &str = "  --------------------------------------";

const TAB_SPACE: &str = "  ";
const SINGLE_SPACE: &str = " ";

pub fn basic_request_executor(
    request: RequestData,
    mut writer_stdout: impl CliWriterRepository + 'static,
    mut writer_stderr: impl CliWriterRepository + 'static,
) -> CommandExecutor {
    Box::new(|provider: Arc<Mutex<dyn Backend>>| {
        tokio::spawn(async move {
            let provider = provider.clone();
            let title = {
                let method =
                    StyledStr::from(request.method.as_str()).with_text_style(TextStyle::Bold);
                let url = StyledStr::from(request.url.as_str()).with_color_text(Color::Blue);

                [
                    StyledStr::from(TAB_SPACE),
                    method,
                    StyledStr::from(SINGLE_SPACE),
                    url,
                ]
            };

            let headers: Vec<[StyledStr; 5]> = {
                request
                    .headers
                    .iter()
                    .map(|(k, v)| {
                        [
                            StyledStr::from(TAB_SPACE),
                            StyledStr::from("| "),
                            StyledStr::from(k),
                            StyledStr::from(":"),
                            StyledStr::from(v),
                        ]
                    })
                    .collect()
            };

            writer_stderr.print_lines([BREAK_LINE]);
            writer_stderr.print_lines_styled([title]);
            writer_stderr.print_lines_styled(headers);
            writer_stderr.print_lines([BREAK_LINE]);

            let request_id = provider.lock().await.add_request(request).await?;
            let response_submit = provider.lock().await.submit_request_async(request_id).await?;
            let (response_submit, mut listener_submit) =
                chain_listener_to_receiver(response_submit);

            // Loading spinner
            {
                let pb = ProgressBar::new(100);
                pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
                pb.set_message("Loading...");

                let mut intv = tokio::time::interval(std::time::Duration::from_millis(14));
                loop {
                    if listener_submit.try_recv().is_ok() {
                        break;
                    }
                    intv.tick().await;
                    pb.inc(1);
                }
                pb.finish_and_clear();
            }

            let response_to_show = response_submit.await?;

            if let Err(err_message) = response_to_show {
                writer_stderr.print_lines_styled([[
                    StyledStr::from(TAB_SPACE),
                    StyledStr::from("Error Requesting...").with_color_text(Color::Red),
                ]]);

                writer_stderr.print_lines([err_message]);
                writer_stderr.print_lines([BREAK_LINE]);

                return anyhow::Ok(());
            }

            let response = response_to_show.unwrap();
            let response_status = response.status.to_string();

            let response_status_message = get_status_code_message(response.status);
            let response_status_message_styled = format!(" ({response_status_message})");

            let title_status = [
                StyledStr::from(TAB_SPACE),
                StyledStr::from("STATUS: ").with_text_style(TextStyle::Bold),
                StyledStr::from(&response_status),
                StyledStr::from(&response_status_message_styled),
            ];
            let headers: Vec<[StyledStr; 5]> = {
                response
                    .headers
                    .iter()
                    .map(|(k, v)| {
                        [
                            StyledStr::from(TAB_SPACE),
                            StyledStr::from("| "),
                            StyledStr::from(k),
                            StyledStr::from(":"),
                            StyledStr::from(v),
                        ]
                    })
                    .collect()
            };

            writer_stderr.print_lines_styled([title_status]);
            writer_stderr.print_lines([BREAK_LINE_WITH_GAP]);
            writer_stderr.print_lines_styled(headers);
            writer_stderr.print_lines([BREAK_LINE_WITH_GAP]);
            writer_stdout.print_lines([response.body]);

            Ok(())
        })
    })
}
