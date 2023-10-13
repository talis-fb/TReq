use crate::services::request::{
    commands::{CommandRequestService as Command, CommandsFactory},
    entity::RequestData,
};

impl CommandsFactory {
    pub fn edit_request(id: String, request: RequestData) -> Command {
        Box::new(|mut service| {
            service.edit_request(id, request);
            Ok(service)
        })
    }

    pub fn add_request(request: RequestData) -> Command {
        Box::new(|mut service| {
            service.add_request(request);
            Ok(service)
        })
    }

    pub fn delete_request(id: String) -> Command {
        Box::new(|mut service| {
            service.delete_request(id);
            Ok(service)
        })
    }
}
