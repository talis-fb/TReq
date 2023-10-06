// use tokio::sync::mpsc;
// use tokio::sync::oneshot;
//
// use crate::utils::commands::Command;
//
// use super::facade::RequestServiceFacade;
//
// pub struct RequestServiceRunner {
//     pub request_service: Option<RequestServiceFacade>,
//     pub commands_listener: mpsc::Receiver<Command<RequestServiceFacade>>,
//     pub shutdown_listener: oneshot::Receiver<()>,
// }
//
// impl RequestServiceRunner {
//     pub async fn listen(&mut self) -> Result<(), ()> {
//
//         loop {
//             tokio::select! {
//                 Some(command) = self.commands_listener.recv() => {
//                     let service_before = self.request_service.take().unwrap();
//                     let output_command = command(service_before);
//
//                     match output_command {
//                         Ok(service) => {
//                             self.request_service = Some(service);
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
//                 _ = &mut self.shutdown_listener => { break }
//             }
//         }
//
//         Ok(())
//     }
// }
