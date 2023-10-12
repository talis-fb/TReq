use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::utils::commands::Command;

pub struct ServiceRunner<ServiceFacade>
where
    ServiceFacade: Sized + Send,
{
    // pub service: ServiceFacade,
    // pub commands_channel: Option<mpsc::Receiver<Command<ServiceFacade>>>,
    // pub cancel_channel: Option<oneshot::Receiver<()>>,

    pub command_channel: mpsc::Sender<Command<ServiceFacade>>,
    shutdown_channel: oneshot::Sender<()>,
}


// impl<T: Send> Drop for ServiceRunner<T> {
//     fn drop(&mut self) {
//         let (tx, _) = oneshot::channel::<()>();
//         let old_channel = std::mem::replace(&mut self.shutdown_channel, tx);
//         let _ = old_channel.send(());
//     }
// }

impl<ServiceFacade: Send + 'static> ServiceRunner<ServiceFacade> {
    pub async fn from(service: ServiceFacade) -> Self {

        let (tx_command_channel, mut rx_command_channel) = mpsc::channel::<Command<ServiceFacade>>(32);
        let (tx_cancel_channel, rx_cancel_channel) = oneshot::channel::<()>();


        let (tx_is_thread_ready, rx_is_thread_ready) = oneshot::channel::<()>();

        tokio::task::spawn(async move {
            let mut service_instance = service;
            let mut cancel_channel = rx_cancel_channel;

            // loop {
            println!("init loop");
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
                Ok(_) = cancel_channel => { 
                    println!("Runner select: BREAKKK");
                },
                else => {
                    println!("Runner select: ELSE");
                }
            }
            // }

            println!("end loop");
        });

        // rx_is_thread_ready.await;

        Self {
            command_channel: tx_command_channel,
            shutdown_channel: tx_cancel_channel,
        }
    }

    // pub async fn listen(&mut self) -> Result<(), ()> {
    //
    //     let cancel_channel = self.cancel_channel.take().unwrap();
    //     let command_channel = self.commands_channel.take().unwrap();
    //     let service = self.service.clone();
    //
    //     tokio::task::spawn(async move {
    //         loop {
    //             tokio::select! {
    //                 Some(command) = command_channel.recv() => {
    //                     let service_before = service;
    //
    //                     let aa = *service_before;
    //
    //                     let output_command = command(aa);
    //
    //                     match output_command {
    //                         Ok(service) => {
    //                             self.service = Some(service);
    //                         },
    //                         Err(_) => {
    //                             // THIS should reset to initial service instance
    //                             // as it's not possible to make a clone in all execution, the
    //                             // error data must contains the backup state of service
    //                             // this way. Each command must return what would be the initial state
    //                             //
    //                             // self.request_service = Some(service_now);
    //                             // throws_error_to_somewhere();
    //                             todo!()
    //                         }
    //                     }
    //                 }
    //                 _ = cancel_channel => { break }
    //             }
    //         }
    //     });
    //
    //     Ok(())
    // }
}
