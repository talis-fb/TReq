use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio::sync::oneshot;

use super::services::files::facade::FileServiceFacade;
use super::services::files::service::FileServiceInstance;
use super::services::web_client::commands::CommandsFactory as WebClientCommandsFactory;
use super::services::web_client::entities::Response;
use super::services::web_client::facade::WebClientFacade;
use super::services::web_client::service::WebClientInstance;
use crate::app::service_commands::Command;
use crate::app::service_runner::ServiceRunner;
use crate::app::services::files::commands::CommandsFactory as FileServiceCommandsFactory;
use crate::app::services::request::commands::CommandsFactory as RequestServCommandsFactory;
use crate::app::services::request::entities::RequestData;
use crate::app::services::request::facade::RequestServiceFacade;
use crate::app::services::request::service::RequestServiceInstance;
use crate::utils::files as file_utils;
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
    async fn find_all_request_name(&mut self) -> Result<Vec<String>>;

    // Pending...
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

#[async_trait]
impl Backend for AppBackend {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        let resp = run_command_waiting_response(
            &self.request_service,
            RequestServCommandsFactory::add_request(request),
        )
        .await?;
        Ok(resp)
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        run_commands(
            &self.request_service,
            [RequestServCommandsFactory::edit_request(id, request)],
        )
        .await?;
        Ok(())
    }
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        let request = run_command_waiting_response(
            &self.request_service,
            RequestServCommandsFactory::get_request_data(id),
        )
        .await?;
        Ok(request)
    }
    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            &self.request_service,
            [RequestServCommandsFactory::delete_request(id)],
        )
        .await?;
        Ok(())
    }
    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            &self.request_service,
            [RequestServCommandsFactory::undo_request_data(id)],
        )
        .await?;
        Ok(())
    }
    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        run_commands(
            &self.request_service,
            [RequestServCommandsFactory::redo_request_data(id)],
        )
        .await?;
        Ok(())
    }

    async fn submit_request_blocking(&mut self, id: UUID) -> Result<Response> {
        let request_data = self
            .get_request(id)
            .await?
            .ok_or(Error::msg("Not found request to given ID"))?;

        let resp = run_command_waiting_response(
            &self.web_client,
            WebClientCommandsFactory::submit((*request_data).clone()),
        )
        .await?;
        Ok(resp.unwrap())
    }

    async fn submit_request_async(
        &mut self,
        id: UUID,
    ) -> Result<oneshot::Receiver<Result<Response, String>>> {
        let request_data = self.get_request(id).await?.unwrap();
        let Command {
            command_fn,
            response,
        } = WebClientCommandsFactory::submit((*request_data).clone());
        self.web_client.command_channel.send(command_fn).await?;
        Ok(response.unwrap())
    }
    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()> {
        let path = run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::get_or_create_data_file(name),
        )
        .await??;

        let request_data = serde_json::to_string(&request_data)?;
        file_utils::write_to_file(path, &request_data).await?;
        Ok(())
    }

    async fn get_request_saved(&mut self, name: String) -> Result<RequestData> {
        let path = run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::get_or_create_data_file(name),
        )
        .await??;

        let request_data = file_utils::read_from_file(path.clone()).await?;
        if request_data.is_empty() {
            run_commands(
                &self.file_service,
                [FileServiceCommandsFactory::remove_file(path)],
            )
            .await?;
            return Err(Error::msg("This request does not exist"));
        }

        let request_data: RequestData = serde_json::from_str(&request_data)?;
        Ok(request_data)
    }

    async fn find_all_request_name(&mut self) -> Result<Vec<String>> {
        let response = run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::find_all_data_files(),
        )
        .await??;
        let file_names = response
            .into_iter()
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        Ok(file_names)
    }
}

async fn run_commands<Service, Resp>(
    service: &ServiceRunner<Service>,
    commands: impl IntoIterator<Item = Command<Service, Resp>>,
) -> Result<()>
where
    Service: Send + 'static,
{
    for Command { command_fn, .. } in commands {
        service.command_channel.send(command_fn).await?;
    }
    Ok(())
}

async fn run_command_waiting_response<Service, Resp>(
    service: &ServiceRunner<Service>,
    command: Command<Service, Resp>,
) -> Result<Resp>
where
    Service: Send + 'static,
{
    let Command {
        command_fn,
        response,
    } = command;

    service.command_channel.send(command_fn).await?;
    if let Some(listener) = response {
        Ok(listener.await?)
    } else {
        Err(Error::msg("No response listener"))
    }
}
