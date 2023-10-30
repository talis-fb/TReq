use super::commands::CommandRequestService as Command;
use super::service::RequestServiceInstance;
use crate::utils::commands::CommandClosureType;

pub type CommandRequestService = CommandClosureType<RequestServiceInstance>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> Command {
        Box::new(|service| Ok(service))
    }
}

pub mod crud;
pub mod history;
