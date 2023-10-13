use async_trait::async_trait;

use crate::services::request::commands::CommandsFactory;
use crate::services::request::entity::RequestData;
use crate::services::request::facade::RequestServiceFacade;
use crate::services::runner::ServiceRunner;

use super::request::service::RequestServiceInstance;

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
#[async_trait]
pub trait Provider {
    async fn add_request(&mut self, request: RequestData);
    async fn edit_request(&mut self, id: String, request: RequestData);
    async fn delete_request(&mut self, id: String);
}

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
pub struct ServicesProvider {
    request_service: ServiceRunner<RequestServiceInstance>,
    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

#[async_trait]
impl Provider for ServicesProvider {
    async fn add_request(&mut self, request: RequestData) {
        let output = self
            .request_service
            .command_channel
            .send(CommandsFactory::add_request(request))
            .await;

        if let Err(err) = output {
            println!("erro: {}", err);
        }
    }
    async fn edit_request(&mut self, id: String, request: RequestData) {
        let output = self
            .request_service
            .command_channel
            .send(CommandsFactory::edit_request(id, request))
            .await;

        if let Err(err) = output {
            println!("erro: {}", err);
        }
    }
    async fn delete_request(&mut self, id: String) {
        let output = self
            .request_service
            .command_channel
            .send(CommandsFactory::delete_request(id))
            .await;

        if let Err(err) = output {
            println!("erro: {}", err);
        }
    }
}

impl ServicesProvider {
    pub async fn init<ServiceRequest>(request_service: ServiceRequest) -> Self
    where
        ServiceRequest: RequestServiceFacade + Send + 'static,
    {
        let request_service = {
            let request_service = Box::new(request_service);
            ServiceRunner::<RequestServiceInstance>::from(request_service).await
        };

        Self { request_service }
    }
}
