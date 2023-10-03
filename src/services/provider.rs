use tokio::sync::mpsc;
use tokio::sync::oneshot;

use super::request::facade::RequestServiceFacade;
use super::runner::ServiceRunner;
use crate::utils::commands::Command;


// Basicamente TODOS os endpoints do app, é de fato a interface para o backend
pub trait Provider {
    fn add_request();
    fn edit_request();
    fn delete_request();
}


struct ServiceWrapper<ServiceFacade> where ServiceFacade : Sized {
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
    fn add_request() {
        todo!()
    }
    fn edit_request() {
        todo!()
    }
    fn delete_request() {
        todo!()
    }
}

impl ServicesProvider {
    pub fn init() -> Self {
        let (tx,rx) = mpsc::channel(32);

        let (tx_o,rx_o) = oneshot::channel();

        Self {
            request_service: ServiceWrapper { 
                runner: ServiceRunner {
                    service: None,
                    commands_channel: rx,
                    cancel_channel: rx_o,
                },
                command_channel: tx,
                shutdown_channel:tx_o,
            }
        }

        
    }
}
