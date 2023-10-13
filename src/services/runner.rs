use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::utils::commands::CommandClosureType;

pub struct ServiceRunner<ServiceFacade>
where
    ServiceFacade: Sized + Send,
{
    pub command_channel: mpsc::Sender<CommandClosureType<ServiceFacade>>,
    shutdown_channel: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

impl<ServiceFacade: Send + 'static> ServiceRunner<ServiceFacade> {
    pub fn close(&mut self) -> Option<()> {
        let (tx, _) = oneshot::channel::<()>();
        let shutdown_channel = std::mem::replace(&mut self.shutdown_channel, tx);
        shutdown_channel.send(()).ok()
    }

    pub async fn from(service: ServiceFacade) -> Self {
        let (tx_command_channel, mut rx_command_channel) =
            mpsc::channel::<CommandClosureType<ServiceFacade>>(32);
        let (tx_cancel_channel, rx_cancel_channel) = oneshot::channel::<()>();

        let (tx_is_command_listener_ready, rx_is_command_listener_ready) = oneshot::channel::<()>();

        let task = tokio::task::spawn(async move {
            let mut service_instance = service;
            let mut cancel_channel = rx_cancel_channel;

            tx_is_command_listener_ready.send(()).unwrap();

            println!("init loop");
            loop {
                tokio::select! {
                    Some(command) = rx_command_channel.recv() => {
                        println!("Runner select: Command");
                        let output_command = command(service_instance);

                        match output_command {
                            Ok(service_result) => {
                                service_instance = service_result;
                            },
                            Err(_) => {
                                // THSend rustIS should reset to initial service instance
                                // as it's not possible to make a clone in all execution, the
                                // error data must contains the backup state of service
                                // this way. Each command must return what would be the initial state
                                //
                                // self.request_service = Some(service_now);
                                // throws_error_to_somewhere();
                                todo!()
                            }
                        }
                    },
                    Ok(_) = &mut cancel_channel => {
                        println!("Runner select: BREAKKK");
                        break;
                    },
                    else => {
                        println!("Runner select: ELSE");
                        break;
                    }
                }
            }

            println!("end loop");
        });

        rx_is_command_listener_ready.await.unwrap();

        Self {
            command_channel: tx_command_channel,
            shutdown_channel: tx_cancel_channel,
            task,
        }
    }
}
