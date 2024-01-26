use super::{CommandRequestService as CommandsService, CommandsFactory};
use crate::app::service_commands::Command;
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::uuid::UUID;

impl CommandsFactory {
    pub fn undo_request_data(id: UUID) -> CommandsService<()> {
        Command::from(move |mut service: RequestServiceInstance| {
            service.undo_request_data(id);
            service
        })
    }
    pub fn redo_request_data(id: UUID) -> CommandsService<()> {
        Command::from(move |mut service: RequestServiceInstance| {
            service.redo_request_data(id);
            service
        })
    }
}
