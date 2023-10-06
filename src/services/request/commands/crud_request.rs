use crate::services::request::{
    commands::{CommandRequestService, Commands},
    entity::RequestData,
    facade::RequestServiceFacade,
};

impl Commands {
    pub fn edit_request(id: String, request: RequestData) -> CommandRequestService {
        Box::new(|mut service: Box<dyn RequestServiceFacade>| {
            service.edit_request(id, request);
            Ok(service)
        })
    }

    pub fn add_request(request: RequestData) -> CommandRequestService {
        Box::new(|mut service: Box<dyn RequestServiceFacade>| {
            service.add_request(request);
            Ok(service)
        })
    }

    pub fn delete_request(id: String) -> CommandRequestService {
        Box::new(|mut service: Box<dyn RequestServiceFacade>| {
            service.delete_request(id);
            Ok(service)
        })
    }
}
