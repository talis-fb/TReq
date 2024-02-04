use std::sync::Arc;

use tokio::sync::oneshot;

use super::super::entities::requests::RequestData;
use super::{CommandRequestService as CommandService, CommandsFactory};
use crate::app::service_commands::Command;
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::uuid::UUID;

impl CommandsFactory {
    pub fn edit_request(id: UUID, request: RequestData) -> CommandService<()> {
        Command::from(move |mut service: RequestServiceInstance| {
            service.edit_request(id, request);
            service
        })
    }

    pub fn add_request(request: RequestData) -> CommandService<UUID> {
        let (tx, rx) = oneshot::channel::<UUID>();

        Command::from(move |mut service: RequestServiceInstance| {
            let id = service.add_request(request);
            tx.send(id).ok();
            service
        })
        .with_response(rx)
    }

    pub fn get_request_data(id: UUID) -> CommandService<Option<Arc<RequestData>>> {
        let (tx, rx) = oneshot::channel::<Option<Arc<RequestData>>>();
        Command::from(move |service: RequestServiceInstance| {
            tx.send(service.get_request_data(id)).ok();
            service
        })
        .with_response(rx)
    }

    pub fn delete_request(id: UUID) -> CommandService<()> {
        CommandService::from(move |mut service| {
            service.delete_request(id);
            service
        })
    }
}
