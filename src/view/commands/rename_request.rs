use std::io::{stdin, BufRead};

use anyhow::Ok;
use async_trait::async_trait;

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

        if !self.has_to_confirm {
            self.writer.print_lines_styled([[
                StyledStr::from(" Are you sure? [y/N] ").with_color_text(Color::Red),
            ]]);

            let mut input = String::new();

            stdin().lock().read_line(&mut input)?;

            while input.trim().to_lowercase() != "y" {
                if input.trim().to_lowercase() == "n" {
                    self.writer.print_lines_styled([[
                        StyledStr::from(" Aborted ").with_color_text(Color::Red),
                    ]]);
                    self.writer.print_lines([BREAK_LINE]);

                    return Ok(());
                }
                
                input = String::new();
                stdin().read_line(&mut input)?;
            }
        }

        provider.rename_request_saved(self.request_name, self.new_name).await?;

        self.writer.print_lines([" Ok "]);

        Ok(())
    }
}