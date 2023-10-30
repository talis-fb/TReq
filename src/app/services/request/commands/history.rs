use super::{CommandRequestService as Command, CommandsFactory};
use crate::utils::uuid::UUID;

impl CommandsFactory {
    pub fn undo_request_data(id: UUID) -> Command {
        Box::new(move |mut service| {
            service.undo_request_data(id);
            Ok(service)
        })
    }
    pub fn redo_request_data(id: UUID) -> Command {
        Box::new(move |mut service| {
            service.redo_request_data(id);
            Ok(service)
        })
    }
}
