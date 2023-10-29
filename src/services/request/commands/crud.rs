use std::sync::Arc;

use tokio::sync::oneshot::{self, Receiver};

use crate::services::request::commands::{CommandRequestService as Command, CommandsFactory};
use crate::services::request::entity::RequestData;
use crate::utils::uuid::UUID;

impl CommandsFactory {
    pub fn edit_request(id: UUID, request: RequestData) -> Command {
        Box::new(move |mut service| {
            service.edit_request(id, request);
            Ok(service)
        })
    }

    pub fn add_request(request: RequestData) -> (Command, Receiver<UUID>) {
        let (tx, rx) = oneshot::channel::<UUID>();

        (
            Box::new(|mut service| {
                let id = service.add_request(request);
                tx.send(id).ok();
                Ok(service)
            }),
            rx,
        )
    }

    pub fn get_request_data(id: UUID) -> (Command, Receiver<Option<Arc<RequestData>>>) {
        let (tx, rx) = oneshot::channel::<Option<Arc<RequestData>>>();
        (
            Box::new(move |service| {
                tx.send(service.get_request_data(id)).ok();
                Ok(service)
            }),
            rx,
        )
    }

    pub fn delete_request(id: UUID) -> Command {
        Box::new(move |mut service| {
            service.delete_request(id);
            Ok(service)
        })
    }
}
