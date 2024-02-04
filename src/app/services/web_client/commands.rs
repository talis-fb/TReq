use tokio::sync::oneshot;

use super::entities::Response;
use super::service::WebClientInstance;
use crate::app::service_commands::Command;
use crate::app::services::request::entities::requests::RequestData;

pub type CommandWebClient<Resp> = Command<WebClientInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn submit(request: RequestData) -> CommandWebClient<anyhow::Result<Response>> {
        let (tx, rx) = oneshot::channel();
        Command::from(move |mut service: WebClientInstance| {
            let task = service.submit_async(request);
            tokio::task::spawn(async move {
                let response = task.await.map_err(|e| e.into()).and_then(|resp| resp);
                tx.send(response).ok();
            });
            service
        })
        .with_response(rx)
    }
}
