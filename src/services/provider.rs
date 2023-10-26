use std::sync::Arc;
use std::vec;

use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::services::request::commands::CommandsFactory;
use crate::services::request::entity::RequestData;
use crate::services::request::facade::RequestServiceFacade;
use crate::services::runner::ServiceRunner;
use crate::utils::uuid::UUID;

use super::request::commands::CommandsUtils;
use super::request::service::RequestServiceInstance;

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
#[async_trait]
pub trait Provider {
    async fn add_request(&mut self, request: RequestData) -> UUID;
    async fn edit_request(&mut self, id: UUID, request: RequestData);
    async fn delete_request(&mut self, id: UUID);
    async fn get_request(&mut self, id: UUID) -> Option<Arc<RequestData>>;
}

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
pub struct ServicesProvider {
    request_service: ServiceRunner<RequestServiceInstance>,
    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

#[async_trait]
impl Provider for ServicesProvider {
    async fn add_request(&mut self, request: RequestData) -> UUID {
        let (command, resp) = CommandsFactory::add_request(request);
        let output = self.request_service.command_channel.send(command).await;
        if let Err(err) = output {
            println!("erro: {}", err);
        }
        resp.await.unwrap()
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) {
        let command = CommandsFactory::edit_request(id, request);
        let output = self.request_service.command_channel.send(command).await;
        if let Err(err) = output {
            println!("erro: {}", err);
        }
    }
    async fn get_request(&mut self, id: UUID) -> Option<Arc<RequestData>> {
        let (command, resp) = CommandsFactory::get_request_data(id);
        let output = self.request_service.command_channel.send(command).await;
        if let Err(err) = output {
            println!("erro: {}", err);
        }
        resp.await.unwrap()
    }
    async fn delete_request(&mut self, id: UUID) {
        let command = CommandsFactory::delete_request(id);
        let output = self.request_service.command_channel.send(command).await;
        if let Err(err) = output {
            println!("erro: {}", err);
        }
    }
}

impl ServicesProvider {
    pub async fn init(request_service: impl RequestServiceFacade + Send + 'static) -> Self {
        let request_service = {
            let request_service = Box::new(request_service);
            ServiceRunner::<RequestServiceInstance>::from(request_service).await
        };

        Self { request_service }
    }
}

// impl ServicesProvider {
//     pub async fn init<ServiceRequest>(request_service: ServiceRequest) -> Self
//     where
//         ServiceRequest: RequestServiceFacade + Send + 'static,
//     {
//         let request_service = {
//             let request_service = Box::new(request_service);
//             ServiceRunner::<RequestServiceInstance>::from(request_service).await
//         };
//
//         Self { request_service }
//     }
// }
