use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::utils::commands::CommandClosureType;

pub struct ServiceRunner<ServiceInstance>
where
    ServiceInstance: Sized + Send,
{
    pub command_channel: mpsc::Sender<CommandClosureType<ServiceInstance>>,
    shutdown_channel: oneshot::Sender<()>,
    tx_error_channel: broadcast::Sender<String>,
    task: JoinHandle<()>,
}

impl<ServiceInstance: Send + 'static> ServiceRunner<ServiceInstance> {
    pub fn from(service: ServiceInstance, service_name: &'static str) -> Self {
        let (tx_command_channel, mut rx_command_channel) =
            mpsc::channel::<CommandClosureType<ServiceInstance>>(32);
        let (tx_error_channel, _) = broadcast::channel::<String>(16);

        let (tx_cancel_channel, rx_cancel_channel) = oneshot::channel::<()>();

        let error_channel = tx_error_channel.clone();
        let task = tokio::task::spawn(async move {
            let mut service_instance = service;
            let mut cancel_channel = rx_cancel_channel;

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
                                eprintln!(" #> Error running command at service {}\n #> {}", service_name, error.error_message);
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
