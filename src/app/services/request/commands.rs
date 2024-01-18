use super::service::RequestServiceInstance;
use crate::utils::commands::Command;

pub type CommandRequestService<Resp> = Command<RequestServiceInstance, Resp>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> CommandRequestService<()> {
        Command::from(|service| Ok(service))
    }
}

pub mod crud;
pub mod history;
