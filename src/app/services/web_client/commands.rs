use tokio::sync::oneshot;

use super::entities::Response;
use super::service::WebClientInstance;
use crate::app::services::request::entities::RequestData;
use crate::utils::commands::Command;

pub type CommandWebClient<Resp> = Command<WebClientInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> CommandWebClient<()> {
        Command::from(|service| Ok(service))
    }

    pub fn submit(request: RequestData) -> CommandWebClient<Result<Response, String>> {
        let (tx, rx) = oneshot::channel();
        Command::from(move |mut service: WebClientInstance| {
            let id_task = service.submit(request);
            let gg = service.get_task_request(id_task);

            if let Some(task) = gg {
                tokio::task::spawn(async move {
                    let response = task.await.map_err(|e| e.to_string());

                    match response {
                        Ok(response) => tx.send(response),
                        Err(err) => tx.send(Err(err)),
                    }
                });
            }
            Ok(service)
        })
        .with_response(rx)
    }
}
