use anyhow::Ok;
use async_trait::async_trait;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct RenameRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub new_name: String,
    pub has_to_confirm: bool,
    pub writer: Writer,
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for RenameRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer.print_lines_styled([[
            StyledStr::from(" Renaming from: ").with_color_text(Color::Red),
            StyledStr::from(&self.request_name).with_color_text(Color::Yellow),
            StyledStr::from(" to: ").with_color_text(Color::Red),
            StyledStr::from(&self.new_name).with_color_text(Color::Yellow),
        ]]);
        self.writer.print_lines([BREAK_LINE]);

        if self.has_to_confirm {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue?")
                .wait_for_newline(true)
                .interact()
                .unwrap()
            {
                self.writer.print_lines([BREAK_LINE]);
            } else {
                return Ok(());
            }
        }

        provider
            .rename_request_saved(self.request_name, self.new_name)
            .await?;

        self.writer.print_lines([" Ok "]);

        Ok(())
    }
}
