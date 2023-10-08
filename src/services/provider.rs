use tokio::sync::mpsc;
use tokio::sync::oneshot;

use super::request::facade::RequestServiceFacade;
use super::runner::ServiceRunner;
use crate::services::request::commands::Commands;
use crate::services::request::entity::RequestData;
use crate::utils::commands::Command;

// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
pub trait Provider {
    fn add_request(&mut self);
    fn edit_request(&mut self);
    fn delete_request(&mut self);
}

struct ServiceWrapper<ServiceFacade>
where
    ServiceFacade: Sized,
{
    runner: ServiceRunner<ServiceFacade>,
    command_channel: mpsc::Sender<Command<ServiceFacade>>,
    shutdown_channel: oneshot::Sender<()>,
}

// ESSE é o cara que vai ter as instancais dos runners de cada serviço
//  ele será a unica depedência (via composição) das Views, no caso, a CLI e TUI
//
//
pub struct ServicesProvider {
    request_service: ServiceWrapper<Box<dyn RequestServiceFacade>>,
    // request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

impl Provider for ServicesProvider {
    fn add_request(&mut self) {

        let req = RequestData::default();

        let command = Commands::add_request(req);

        let sender = self.request_service.command_channel.clone();

        tokio::task::spawn(async move {
            sender.send(command).await.unwrap();
        });
    }
    fn edit_request(&mut self) {
        todo!()
    }
    fn delete_request(&mut self) {
        todo!()
    }
}

impl ServicesProvider {
    pub fn init<ServiceRequest>(request_service: ServiceRequest) -> Self
    where
        ServiceRequest: RequestServiceFacade + 'static,
    {
        let request_service = {
            let request_service = Box::new(request_service) as Box<dyn RequestServiceFacade>;
            let (tx_command_channel, rx_command_channel) = mpsc::channel(32);
            let (tx_cancel_channel, rx_cancel_channel) = oneshot::channel();

            let service_runner = ServiceRunner {
                service: Some(request_service),
                commands_channel: rx_command_channel,
                cancel_channel: rx_cancel_channel,
            };

            ServiceWrapper {
                runner: service_runner,
                command_channel: tx_command_channel,
                shutdown_channel: tx_cancel_channel,
            }
        };

        Self { request_service }
    }
}
