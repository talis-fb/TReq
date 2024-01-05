use async_trait::async_trait;

use crate::app::provider::Provider;
use crate::app::services::request::entity::RequestData;
use crate::app::services::web_client::entity::get_status_code_message;
use crate::utils::observable::chain_listener_to_receiver;
use crate::view::cli::writer::CliWriterRepository;
use crate::view::commands::AppCommandExecutor;
use crate::view::style::{create_vec_styled_string_from, Color, StyledStr, TextStyle};

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle, ProgressState};

#[derive(Debug)]
pub struct BasicRequestExecutor<Writer>
where
    Writer: CliWriterRepository + Send,
{
    pub req: RequestData,
    pub writer_stdout: Writer,
    pub writer_stderr: Writer,
}

#[async_trait]
impl<W> AppCommandExecutor for BasicRequestExecutor<W>
where
    W: CliWriterRepository + Send,
{
    async fn execute(&mut self, mut provider: Box<dyn Provider + Send>) -> anyhow::Result<()> {
        const BREAK_LINE: &str = "----------------------------------------";
        const BREAK_LINE_WITH_GAP: &str = "  ------------------------------------";

        const TAB_SPACE: &str = "  ";
        const SINGLE_SPACE: &str = " ";

        let tab_space_styled = StyledStr::from(TAB_SPACE);
        let single_space_styled = StyledStr::from(SINGLE_SPACE);

        let title = {
            let method =
                StyledStr::from(self.req.method.as_str()).with_text_style(TextStyle::Bold);
            let url = StyledStr::from(self.req.url.as_str()).with_color_text(Color::Blue);

            [
                tab_space_styled.clone(),
                method,
                single_space_styled.clone(),
                url,
            ]
        };

        let headers: Vec<[StyledStr; 5]> = {
            self.req
                .headers
                .iter()
                .map(|(k, v)| {
                    [
                        tab_space_styled.clone(),
                        StyledStr::from("| "),
                        StyledStr::from(k),
                        StyledStr::from(":"),
                        StyledStr::from(v),
                    ]
                })
                .collect()
        };

        self.writer_stderr.print_lines([BREAK_LINE]);
        self.writer_stderr.print_lines_styled([title]);
        self.writer_stderr.print_lines_styled(headers);
        self.writer_stderr.print_lines([BREAK_LINE]);

        let request_to_do = std::mem::take(&mut self.req);
        let id = provider.add_request(request_to_do).await?;

        let response_submit = provider.submit_request_async(id).await?;
        let (response_submit, mut listener_submit) = chain_listener_to_receiver(response_submit);

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
            self.writer_stderr.print_lines_styled([[
                tab_space_styled.clone(),
                StyledStr::from("Error Requesting...").with_color_text(Color::Red),
            ]]);

            self.writer_stderr.print_lines([err_message]);
            self.writer_stderr.print_lines([BREAK_LINE]);

            return Ok(());
        }

        let response = response_to_show.unwrap();
        let response_status = response.status.to_string();

        let response_status_message = get_status_code_message(response.status);
        let response_status_message_styled = format!(" ({response_status_message})");

        let title_status = [
            tab_space_styled.clone(),
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
                        tab_space_styled.clone(),
                        StyledStr::from("| "),
                        StyledStr::from(k),
                        StyledStr::from(":"),
                        StyledStr::from(v),
                    ]
                })
                .collect()
        };

        self.writer_stderr.print_lines_styled([title_status]);
        self.writer_stderr.print_lines([BREAK_LINE_WITH_GAP]);
        self.writer_stderr.print_lines_styled(headers);
        self.writer_stderr.print_lines([BREAK_LINE_WITH_GAP]);

        self.writer_stdout.print_lines([response.body]);

        Ok(())
    }
}
