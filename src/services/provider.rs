use super::request::facade::RequestServiceFacade;
use super::runner::ServiceRunner;
use crate::services::request::commands::Commands;
use crate::services::request::entity::RequestData;

use async_trait::async_trait;

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
#[async_trait]
pub trait Provider {
    async fn add_request(&mut self);
    async fn edit_request(&mut self);
    async fn delete_request(&mut self);
}

// struct ServiceWrapper<ServiceFacade>
// where
//     ServiceFacade: Sized,
// {
//     runner: ServiceRunner<ServiceFacade>,
//     command_channel: mpsc::Sender<Command<ServiceFacade>>,
//     shutdown_channel: Option<oneshot::Sender<()>>,
// }

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
//
//
pub struct ServicesProvider {
    // request_service: ServiceWrapper<Box<dyn RequestServiceFacade>>,
    request_service: ServiceRunner<Box<dyn RequestServiceFacade + Send>>,

    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

#[async_trait]
impl Provider for ServicesProvider {
    async fn add_request(&mut self) {
        let req = RequestData::default();

        let command = Commands::add_request(req);

        let sender = self.request_service.command_channel.clone();

        let e = sender.send(command).await;

        if let Err(err) = e {
            println!("erro: {}", err);
        }
    }
    async fn edit_request(&mut self) {
        todo!()
    }
    async fn delete_request(&mut self) {
        todo!()
    }
}

impl ServicesProvider {
    pub async fn init<ServiceRequest>(request_service: ServiceRequest) -> Self
    where
        ServiceRequest: RequestServiceFacade + Send + 'static,
    {
        let request_service = {
            let request_service = Box::new(request_service) as Box<dyn RequestServiceFacade + Send>;
            ServiceRunner::from(request_service).await
        };

        Self { request_service }
    }

}
