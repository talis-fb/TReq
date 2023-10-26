use crate::services::request::service::RequestServiceInstance;
use crate::utils::commands::CommandClosureType;

use crate::services::request::commands::CommandRequestService as Command;

pub type CommandRequestService = CommandClosureType<RequestServiceInstance>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> Command {
        Box::new(|service| Ok(service))
    }
}

pub struct CommandsUtils;
impl CommandsUtils {
    pub fn chain(
        iters_commands: impl IntoIterator<Item = Command> + Send + Sync + 'static,
    ) -> Command {
        Box::new(|service| {
            iters_commands
                .into_iter()
                .try_fold(service, |acc, command| command(acc))
        })
        // Box::new(|service| {
        //     let mut output = service;
        //     for command in iters_commands.into_iter() {
        //         output = command(output)?;
        //     }
        //     Ok(output)
        // })
    }
}

pub mod crud_request;
