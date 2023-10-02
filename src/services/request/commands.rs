use crate::services::request::facade::RequestServiceFacade;
use crate::utils::commands::Command as CommandAlias;

pub type Command = CommandAlias<dyn RequestServiceFacade>;

pub struct Commands;

pub mod edit_request;
