use super::{CommandRequestService, CommandsFactory};
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::commands::Command;
use crate::utils::uuid::UUID;

impl CommandsFactory {
    pub fn undo_request_data(id: UUID) -> CommandRequestService<()> {
        Command::from(move |mut service: RequestServiceInstance| {
            service.undo_request_data(id);
            Ok(service)
        })
    }
    pub fn redo_request_data(id: UUID) -> CommandRequestService<()> {
        Command::from(move |mut service: RequestServiceInstance| {
            service.redo_request_data(id);
            Ok(service)
        })
    }
}
