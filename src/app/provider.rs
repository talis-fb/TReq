use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use super::services::web_client::commands::CommandsFactory as WebClientCommandsFactory;
use super::services::web_client::entity::Response;
use super::services::web_client::facade::WebClientFacade;
use super::services::web_client::service::WebClientInstance;
use crate::app::service_runner::ServiceRunner;
use crate::app::services::request::commands::CommandsFactory;
use crate::app::services::request::entity::RequestData;
use crate::app::services::request::facade::RequestServiceFacade;
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::uuid::UUID;

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
pub struct AppProvider {
    request_service: ServiceRunner<RequestServiceInstance>,
    web_client: ServiceRunner<WebClientInstance>,
    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

impl AppProvider {
    pub async fn init(
        request_service: impl RequestServiceFacade + Send + 'static,
        web_client: impl WebClientFacade + Send + 'static,
    ) -> Self {
        let request_service = {
            let request_service = Box::new(request_service);
            ServiceRunner::<RequestServiceInstance>::from(request_service).await
        };

        let web_client = {
            let web_client = Box::new(web_client);
            ServiceRunner::<WebClientInstance>::from(web_client).await
        };

        Self {
            request_service,
            web_client,
        }
    }
}

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
#[async_trait]
pub trait Provider {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID>;
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()>;
    async fn delete_request(&mut self, id: UUID) -> Result<()>;
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>>;
    async fn undo_request(&mut self, id: UUID) -> Result<()>;
    async fn redo_request(&mut self, id: UUID) -> Result<()>;
    async fn submit_request(&mut self, id: UUID) -> Result<Response>;
}

#[async_trait]
impl Provider for AppProvider {
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
    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        let command = CommandsFactory::undo_request_data(id);
        self.request_service.command_channel.send(command).await?;
        Ok(())
    }
    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        let command = CommandsFactory::redo_request_data(id);
        self.request_service.command_channel.send(command).await?;
        Ok(())
    }

    async fn submit_request(&mut self, id: UUID) -> Result<Response> {
        // TODO: Remove this unwrap to a Option -> Result
        let request_data = self.get_request(id).await?.unwrap();
        let (command, resp) = WebClientCommandsFactory::submit((*request_data).clone());
        self.web_client.command_channel.send(command).await?;
        Ok(resp.await?.unwrap())
    }
}
