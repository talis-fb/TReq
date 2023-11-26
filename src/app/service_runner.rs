use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::utils::commands::CommandClosureType;

pub struct ServiceRunner<ServiceInstance>
where
    ServiceInstance: Sized + Send,
{
    pub command_channel: mpsc::Sender<CommandClosureType<ServiceInstance>>,
    shutdown_channel: oneshot::Sender<()>,
    task: JoinHandle<()>,

    // Errors
    pub tx_error_channel: broadcast::Sender<String>,
}

impl<ServiceInstance: Send + 'static> ServiceRunner<ServiceInstance> {
    pub async fn from(service: ServiceInstance) -> Self {
        let (tx_command_channel, mut rx_command_channel) =
            mpsc::channel::<CommandClosureType<ServiceInstance>>(32);
        let (tx_error_channel, _) = broadcast::channel::<String>(16);

        let (tx_cancel_channel, rx_cancel_channel) = oneshot::channel::<()>();

        let (tx_is_command_listener_ready, rx_is_command_listener_ready) = oneshot::channel::<()>();

        let error_channel = tx_error_channel.clone();
        let task = tokio::task::spawn(async move {
            let mut service_instance = service;
            let mut cancel_channel = rx_cancel_channel;

            tx_is_command_listener_ready.send(()).unwrap();

            loop {
                tokio::select! {
                    Some(command) = rx_command_channel.recv() => {
                        let output_command = command(service_instance);

                        match output_command {
                            Ok(service_result) => {
                                service_instance = service_result;
                            },
                            Err(error) => {
                                service_instance = error.snapshot;
                                let _ = error_channel.send(error.error_message);
                            }
                        }
                    },
                    Ok(_) = &mut cancel_channel => {
                        break;
                    },
                    else => {
                        break;
                    }
                }
            }
        });

        rx_is_command_listener_ready.await.unwrap();

        Self {
            command_channel: tx_command_channel,
            shutdown_channel: tx_cancel_channel,
            tx_error_channel,
            task,
        }
    }

    pub fn get_error_listener(&self) -> broadcast::Receiver<String> {
        self.tx_error_channel.subscribe()
    }

    pub fn close(&mut self) -> Option<()> {
        let (tx, _) = oneshot::channel::<()>();
        let shutdown_channel = std::mem::replace(&mut self.shutdown_channel, tx);
        shutdown_channel.send(()).ok()?;
        self.task.abort();
        Some(())
    }
}
