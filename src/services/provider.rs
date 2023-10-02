use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::utils::commands::Command;

use super::request::facade::RequestServiceFacade;
use super::request::RequestService;

pub struct ServicesProvider {
    request_service: RequestService,
    request_service_channel: Receiver<Command<dyn RequestServiceFacade>>,
}

impl ServicesProvider {
    pub fn get_request_service(&self) -> &RequestService {
        &self.request_service
    }

    fn start_request_service(&mut self) -> Result<(), ()> {
        match &self.request_service_channel {
            ServiceReceiver::Idle(receiver) => {
                self.request_service_channel = ServiceReceiver::Listening;
                tokio::task::spawn(async move { while let Some(msg) = receiver.recv().await {} });
            }
            _ => {}
        }

        Ok(())
    }
}
