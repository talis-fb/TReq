use async_trait::async_trait;

use super::ViewCommand;
use crate::app::backend::Backend;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::view::output::utils::BREAK_LINE;
use crate::view::output::writer::CliWriterRepository;
use crate::view::style::{Color, StyledStr};

pub struct SaveRequestWithBaseRequestExecutor<Writer: CliWriterRepository> {
    pub request_name: String,
    pub base_request_name: Option<String>,
    pub input_request_data: PartialRequestData,
    pub writer: Writer,
}

#[async_trait]
impl<Writer: CliWriterRepository> ViewCommand for SaveRequestWithBaseRequestExecutor<Writer> {
    async fn execute(mut self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()> {
        self.writer.print_lines([BREAK_LINE]);
        self.writer
            .print_lines_styled([[StyledStr::from(" Saving").with_color_text(Color::Yellow)]]);
        self.writer.print_lines_styled([[
            StyledStr::from(" -> "),
            StyledStr::from(&self.request_name).with_color_text(Color::Blue),
        ]]);

        let base_request_data = match self.base_request_name {
            Some(base_request_name) => Some(provider.get_request_saved(base_request_name).await?),
            None => None,
        };

        let request_data_to_save = match base_request_data {
            Some(request_data) => request_data.merge(self.input_request_data),
            None => self.input_request_data.to_request_data(),
        };

        provider
            .save_request_datas_as(self.request_name, request_data_to_save)
            .await?;

        Ok(())
    }
}
