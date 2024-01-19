use super::service::RequestServiceInstance;
use crate::app::service_commands::Command;

pub type CommandRequestService<Resp> = Command<RequestServiceInstance, Resp>;

pub struct CommandsFactory;

pub mod crud;
pub mod history;
