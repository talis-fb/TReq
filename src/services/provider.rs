use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use super::request::service::RequestServiceInstance;
use crate::services::request::commands::CommandsFactory;
use crate::services::request::entity::RequestData;
use crate::services::request::facade::RequestServiceFacade;
use crate::services::runner::ServiceRunner;
use crate::utils::uuid::UUID;

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
pub struct ServicesProvider {
    request_service: ServiceRunner<RequestServiceInstance>,
    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
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

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
#[async_trait]
pub trait Provider {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID>;
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()>;
    async fn delete_request(&mut self, id: UUID) -> Result<()>;
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>>;
    async fn rollback_request(&mut self, id: UUID) -> Result<()>;
}

#[async_trait]
impl Provider for ServicesProvider {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        let (command, resp) = CommandsFactory::add_request(request);
        self.request_service.command_channel.send(command).await?;
        Ok(resp.await?)
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        let command = CommandsFactory::edit_request(id, request);
        self.request_service.command_channel.send(command).await?;
        Ok(())
    }
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        let (command, resp) = CommandsFactory::get_request_data(id);
        self.request_service.command_channel.send(command).await?;
        Ok(resp.await?)
    }
    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        let command = CommandsFactory::delete_request(id);
        self.request_service.command_channel.send(command).await?;
        Ok(())
    }
    async fn rollback_request(&mut self, id: UUID) -> Result<()> {
        let command = CommandsFactory::rollback_request_data(id);
        self.request_service.command_channel.send(command).await?;
        Ok(())
    }
}
