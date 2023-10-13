use crate::services::request::facade::RequestServiceFacade;
use crate::services::request::service::RequestServiceInstance;
use crate::utils::commands::CommandClosureType;

pub type CommandRequestService = CommandClosureType<RequestServiceInstance>;

pub struct CommandsFactory;

pub mod crud_request;
