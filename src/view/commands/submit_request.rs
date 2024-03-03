use std::io::{empty, stderr, stdout};

use async_trait::async_trait;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::RequestData;
use crate::app::services::web_client::entities::get_status_code_message;
use crate::utils::channels::chain_listener_to_receiver;
use crate::view::input::cli_input::ViewOptions;
use crate::view::output::utils::{BREAK_LINE, BREAK_LINE_WITH_GAP, SINGLE_SPACE, TAB_SPACE};
use crate::view::output::writer::{CliWriterRepository, CrosstermCliWriter};
use crate::view::style::{Color, StyledStr, TextStyle};

pub struct BasicRequestExecutor<W1, W2, W3>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
    W3: CliWriterRepository,
{
    pub request: RequestData,
    pub progress_draw_target: ProgressDrawTarget,
    pub writer_metadata: W1,
    pub writer_response: W2,
    pub writer_stderr: W3,
}

impl BasicRequestExecutor<CrosstermCliWriter, CrosstermCliWriter, CrosstermCliWriter> {
    pub fn new(request: RequestData, view_options: &ViewOptions) -> Self {
        if view_options.print_body_only {
            BasicRequestExecutor {
                request,
                progress_draw_target: ProgressDrawTarget::stderr(),
                writer_metadata: CrosstermCliWriter::from(empty()),
                writer_response: CrosstermCliWriter::from(stdout()),
                writer_stderr: CrosstermCliWriter::from(stderr()),
            }
        } else if view_options.suppress_output {
            BasicRequestExecutor {
                request,
                progress_draw_target: ProgressDrawTarget::hidden(),
                writer_metadata: CrosstermCliWriter::from(empty()),
                writer_response: CrosstermCliWriter::from(empty()),
                writer_stderr: CrosstermCliWriter::from(stderr()),
            }
        } else {
            BasicRequestExecutor {
                request,
                progress_draw_target: ProgressDrawTarget::stderr(),
                writer_metadata: CrosstermCliWriter::from(stderr()),
                writer_response: CrosstermCliWriter::from(stdout()),
                writer_stderr: CrosstermCliWriter::from(stderr()),
            }
        }
    }
}

#[async_trait]
impl<W1, W2, W3> ViewCommand for BasicRequestExecutor<W1, W2, W3>
where
    W1: CliWriterRepository,
    W2: CliWriterRepository,
    W3: CliWriterRepository,
{
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        let url = self.request.url.to_string();

        let title = {
            let method =
                StyledStr::from(self.request.method.as_str()).with_text_style(TextStyle::Bold);
            let url = StyledStr::from(&url).with_color_text(Color::Blue);

            [
                StyledStr::from(TAB_SPACE),
                method,
                StyledStr::from(SINGLE_SPACE),
                url,
            ]
        };

        let headers: Vec<[StyledStr; 5]> = {
            self.request
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

        self.writer_metadata.print_lines([BREAK_LINE]);
        self.writer_metadata.print_lines_styled([title]);
        self.writer_metadata.print_lines_styled(headers);
        self.writer_metadata.print_lines([BREAK_LINE]);

        let request_id = provider.add_request(self.request).await?;
        let response_submit = provider.submit_request_async(request_id).await?;
        let (response_submit, mut listener_submit) = chain_listener_to_receiver(response_submit);

        // Loading spinner
        {
            let now = tokio::time::Instant::now();

            let pb = ProgressBar::new(100);
            pb.set_draw_target(self.progress_draw_target);
            pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
            pb.set_message("Loading...\t\t 0 MS");

            let mut intv = tokio::time::interval(std::time::Duration::from_millis(14));
            loop {
                if listener_submit.try_recv().is_ok() {
                    break;
                }
                intv.tick().await;
                pb.inc(1);

                let elapsed = format!(" {} MS ", now.elapsed().as_millis());
                pb.set_message("Loading...\t\t".to_owned() + elapsed.as_str());
            }
            pb.finish_and_clear();
        }

        let response_to_show = response_submit.await?;

        if let Err(err_message) = response_to_show {
            self.writer_stderr.print_lines_styled([[
                StyledStr::from(TAB_SPACE),
                StyledStr::from("Error Requesting...").with_color_text(Color::Red),
            ]]);

            self.writer_stderr.print_lines([err_message]);
            self.writer_stderr.print_lines([BREAK_LINE]);

            return anyhow::Ok(());
        }

        let response = response_to_show.unwrap();
        let response_status = response.status.to_string();

        let response_time = format!(" {} MS ", response.response_time_ms);

        let response_status_message = get_status_code_message(response.status);
        let response_status_message_styled = format!(" ({response_status_message})");

        let title_status = [
            StyledStr::from(TAB_SPACE),
            StyledStr::from("STATUS: ").with_text_style(TextStyle::Bold),
            StyledStr::from(&response_status),
            StyledStr::from(&response_status_message_styled),
            StyledStr::from("    "),
            StyledStr::from(&response_time),
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

        self.writer_metadata.print_lines_styled([title_status]);
        self.writer_metadata.print_lines([BREAK_LINE_WITH_GAP]);
        self.writer_metadata.print_lines_styled(headers);
        self.writer_metadata.print_lines([BREAK_LINE_WITH_GAP]);
        self.writer_response.print_lines([response.body]);

        Ok(())
    }
}
