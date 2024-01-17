use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};

use super::services::files::facade::FileServiceFacade;
use super::services::files::service::FileServiceInstance;
use super::services::web_client::commands::CommandsFactory as WebClientCommandsFactory;
use super::services::web_client::entities::Response;
use super::services::web_client::facade::WebClientFacade;
use super::services::web_client::service::WebClientInstance;
use crate::app::service_runner::ServiceRunner;
use crate::app::services::files::commands::CommandsFactory as FileServiceCommandsFactory;
use crate::app::services::request::commands::CommandsFactory;
use crate::app::services::request::entities::RequestData;
use crate::app::services::request::facade::RequestServiceFacade;
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::files::file_utils;
use crate::utils::uuid::UUID;

#[async_trait]
pub trait Backend: Send {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID>;
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()>;
    async fn delete_request(&mut self, id: UUID) -> Result<()>;
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>>;
    async fn undo_request(&mut self, id: UUID) -> Result<()>;
    async fn redo_request(&mut self, id: UUID) -> Result<()>;
    async fn submit_request_blocking(&mut self, id: UUID) -> Result<Response>;
    async fn submit_request_async(
        &mut self,
        id: UUID,
    ) -> Result<oneshot::Receiver<Result<Response, String>>>;

    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()>;
    async fn get_request_saved(&mut self, name: String) -> Result<RequestData>;
    // Pending...
    // async fn get_all_files_in_folder(&mut self, names: &[str]) -> Result<Vec<String>>;
    // async fn remove_request_saved(&mut self, name: String) -> Result<()>;
    // async fn rename_request_saved(&mut self, name: String, new_name: String) -> Result<()>;
}

pub struct AppBackend {
    request_service: ServiceRunner<RequestServiceInstance>,
    web_client: ServiceRunner<WebClientInstance>,
    file_service: ServiceRunner<FileServiceInstance>,
}

impl AppBackend {
    pub fn init(
        request_service: impl RequestServiceFacade + 'static,
        web_client: impl WebClientFacade + 'static,
        file_service: impl FileServiceFacade + 'static,
    ) -> Self {
        let request_service = {
            let request_service = Box::new(request_service);
            ServiceRunner::<RequestServiceInstance>::from(request_service, "RequestService")
        };

        let web_client = {
            let web_client = Box::new(web_client);
            ServiceRunner::<WebClientInstance>::from(web_client, "WebClientService")
        };

        let file_service = {
            let file_service = Box::new(file_service);
            ServiceRunner::<FileServiceInstance>::from(file_service, "FileService")
        };

        Self {
            request_service,
            web_client,
            file_service,
        }
    }
}

async fn run_commands<CommandFn>(
    commands: impl IntoIterator<Item = CommandFn>,
    sender: &mpsc::Sender<CommandFn>,
) -> Result<()>
where
    CommandFn: Sync + Send + 'static,
{
    for c in commands {
        sender.send(c).await?;
    }
    Ok(())
}

async fn run_command_with_response<CommandFn, Response>(
    command: (CommandFn, oneshot::Receiver<Response>),
    sender: &mpsc::Sender<CommandFn>,
) -> Result<Response>
where
    CommandFn: Sync + Send + 'static,
{
    let (command, resp) = command;
    sender.send(command).await?;
    Ok(resp.await?)
}

#[async_trait]
impl Backend for AppBackend {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        let resp = run_command_with_response(
            CommandsFactory::add_request(request),
            &self.request_service.command_channel,
        )
        .await?;
        Ok(resp)
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        run_commands(
            [CommandsFactory::edit_request(id, request)],
            &self.request_service.command_channel,
        )
        .await?;
        Ok(())
    }
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        let (command, resp) = CommandsFactory::get_request_data(id);
        self.request_service.command_channel.send(command).await?;
        Ok(resp.await?)
    }
    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            [CommandsFactory::delete_request(id)],
            &self.request_service.command_channel,
        )
        .await?;
        Ok(())
    }
    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            [CommandsFactory::undo_request_data(id)],
            &self.request_service.command_channel,
        )
        .await?;
        Ok(())
    }
    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            [CommandsFactory::redo_request_data(id)],
            &self.request_service.command_channel,
        )
        .await?;
        Ok(())
    }

    async fn submit_request_blocking(&mut self, id: UUID) -> Result<Response> {
        // TODO: Remove this unwrap to a Option -> Result
        let request_data = self.get_request(id).await?.unwrap();
        let resp = run_command_with_response(
            WebClientCommandsFactory::submit((*request_data).clone()),
            &self.web_client.command_channel,
        )
        .await?;
        Ok(resp.unwrap())
    }

    async fn submit_request_async(
        &mut self,
        id: UUID,
    ) -> Result<oneshot::Receiver<Result<Response, String>>> {
        let request_data = self.get_request(id).await?.unwrap();
        let (command, resp) = WebClientCommandsFactory::submit((*request_data).clone());
        self.web_client.command_channel.send(command).await?;
        Ok(resp)
    }
    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()> {
        let path = run_command_with_response(
            FileServiceCommandsFactory::get_or_create_data_file(name),
            &self.file_service.command_channel,
        )
        .await?;

        let request_data = serde_json::to_string(&request_data)?;
        file_utils::write_to_file(path, &request_data).await?;
        Ok(())
    }

    async fn get_request_saved(&mut self, name: String) -> Result<RequestData> {
        let path = run_command_with_response(
            FileServiceCommandsFactory::get_or_create_data_file(name),
            &self.file_service.command_channel,
        )
        .await?;

        let request_data = file_utils::read_from_file(path.clone()).await?;
        if request_data.is_empty() {
            run_commands(
                [FileServiceCommandsFactory::remove_file(path)],
                &self.file_service.command_channel,
            )
            .await?;
            return Err(Error::msg("This request does not exist"));
        }

        let request_data: RequestData = serde_json::from_str(&request_data)?;
        Ok(request_data)
    }
}
