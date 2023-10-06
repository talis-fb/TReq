use crate::services::request::facade::RequestServiceFacade;
use crate::utils::commands::Command as CommandAlias;

pub type CommandRequestService = CommandAlias<Box<dyn RequestServiceFacade>>;

pub struct Commands;

pub mod crud_request;
