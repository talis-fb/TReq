use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::RequestData;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SaveNewRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub request_data: RequestData,
    pub writer: Writer,
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for SaveNewRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer
            .print_lines_styled([[StyledStr::from(" Saving").with_color_text(Color::Yellow)]]);
        self.writer.print_lines_styled([[
            StyledStr::from(" -> "),
            StyledStr::from(&self.request_name).with_color_text(Color::Blue),
        ]]);

        provider
            .save_request_datas_as(self.request_name, self.request_data)
            .await?;

        Ok(())
    }
}
