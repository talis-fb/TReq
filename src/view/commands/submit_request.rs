use async_trait::async_trait;

use crate::app::provider::Provider;
use crate::app::services::request::entity::RequestData;
use crate::utils::observable::observable;
use crate::view::cli::writer::CliWriterRepository;
use crate::view::commands::AppCommandExecutor;
use crate::view::style::{create_vec_styled_string_from, Color, StyledString, TextStyle};

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
        self.writer_stderr.print_lines_styled([Vec::from([
            StyledString::from(self.req.method.to_string()).with_text_style(TextStyle::Bold),
            StyledString::from(" "),
            StyledString::from(self.req.url.clone()).with_color_text(Color::Blue),
        ])]);

        let request_to_do = std::mem::take(&mut self.req);
        let id = provider.add_request(request_to_do).await?;

        let response_submit = provider.submit_request_async(id).await?;
        let (response_submit, notify_submit) = observable(response_submit);

        let loading_sprites = [
            "⣾ Loading.",
            "⣽ Loading.",
            "⣻ Loading..",
            "⢿ Loading..",
            "⡿ Loading..",
            "⣟ Loading...",
            "⣯ Loading...",
            "⣷ Loading...",
        ];

        self.writer_stderr.print_animation_single_line(
            loading_sprites,
            std::time::Duration::from_millis(300),
            notify_submit,
        );

        let response_to_show = response_submit.await?;

        if let Err(err_message) = response_to_show {
            self.writer_stderr.print_lines_styled([
                Vec::from([
                    "-------------- ".into(),
                    StyledString::from("ERROR").with_color_text(Color::Red),
                    " -------------------".into(),
                ]),
                create_vec_styled_string_from([err_message]),
                create_vec_styled_string_from(["----------------------------------------"]),
            ]);

            return Ok(());
        }

        let response = response_to_show.unwrap();


        self.writer_stderr.print_lines_styled([
            create_vec_styled_string_from(["----------------------------------------"]),
            Vec::from([
                StyledString::from(" STATUS: "),
                StyledString::from(response.status.to_string()).with_color_text({
                    match response.status {
                        0..=299 => Color::Blue,
                        300..=499 => Color::Yellow,
                        500..=1000 => Color::Red,
                        _ => Color::Red,
                    }
                }),
            ]),
            create_vec_styled_string_from(["----------------------------------------"]),
        ]);

        self.writer_stdout.print_lines([response.body]);

        Ok(())
    }
}
