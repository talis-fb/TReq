use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::utils::commands::Command;

pub struct ServiceRunner<ServiceFacade>
where
    ServiceFacade: Sized,
{
    pub service: Option<ServiceFacade>,
    pub commands_channel: mpsc::Receiver<Command<ServiceFacade>>,
    pub cancel_channel: oneshot::Receiver<()>,
}

impl<ServiceFacade> ServiceRunner<ServiceFacade> {
    pub async fn listen(&mut self) -> Result<(), ()> {
        loop {
            tokio::select! {
                Some(command) = self.commands_channel.recv() => {
                    let service_before = self.service.take().unwrap();
                    let output_command = command(service_before);

                    match output_command {
                        Ok(service) => {
                            self.service = Some(service);
                        },
                        Err(_) => {
                            // THIS should reset to initial service instance
                            // as it's not possible to make a clone in all execution, the
                            // error data must contains the backup state of service
                            // this way. Each command must return what would be the initial state
                            //
                            // self.request_service = Some(service_now);
                            // throws_error_to_somewhere();
                            todo!()
                        }
                    }
                }
                _ = &mut self.cancel_channel => { break }
            }
        }

        Ok(())
    }
}
