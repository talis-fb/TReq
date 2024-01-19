use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::app::service_commands::Functor;

pub struct ServiceRunner<ServiceInstance>
where
    ServiceInstance: Sized + Send,
{
    pub service_name: String,
    pub command_channel: mpsc::Sender<Functor<ServiceInstance>>,
    shutdown_channel: oneshot::Sender<()>,
    runner_thread: JoinHandle<()>,
}

impl<ServiceInstance: Send + 'static> ServiceRunner<ServiceInstance> {
    pub fn from(mut service_instance: ServiceInstance, service_name: &'static str) -> Self {
        let (tx_shutdown_channel, rx_shutdown_channel) = oneshot::channel::<()>();
        let (tx_command_channel, mut rx_command_channel) =
            mpsc::channel::<Functor<ServiceInstance>>(32);

        let runner_thread = tokio::task::spawn(async move {
            let mut rx_shutdown_channel = rx_shutdown_channel;
            loop {
                tokio::select! {
                    Some(command_fn) = rx_command_channel.recv() => {
                        service_instance = command_fn(service_instance);
                    },
                    Ok(_) = &mut rx_shutdown_channel => {
                        break;
                    },
                    else => {
                        break;
                    }
                }
            }
        });

        Self {
            service_name: service_name.into(),
            command_channel: tx_command_channel,
            shutdown_channel: tx_shutdown_channel,
            runner_thread,
        }
    }

    pub fn close(self) -> Option<()> {
        self.shutdown_channel.send(()).ok()?;
        self.runner_thread.abort();
        Some(())
    }
}
