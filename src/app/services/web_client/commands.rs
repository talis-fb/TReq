use tokio::sync::oneshot::{self, Receiver};

// use super::commands::CommandRequestService as Command;
use super::{entity::Response, service::WebClientInstance};
use crate::app::services::request::entity::RequestData;
use crate::utils::commands::CommandClosureType;

pub type CommandWebClient = CommandClosureType<WebClientInstance>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> CommandWebClient {
        Box::new(|service| Ok(service))
    }

    pub fn submit(request: RequestData) -> (CommandWebClient, Receiver<Result<Response, String>>) {
        let (tx, rx) = oneshot::channel();
        (
            Box::new(move |mut service| {
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
            }),
            rx,
        )
    }
}
