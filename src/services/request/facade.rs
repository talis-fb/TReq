use super::entity::{RequestData, RequestEntity};
use super::RequestService;
use std::rc::Rc;

pub trait RequestServiceFacade {
    fn add_request(&mut self, request: RequestData) -> String;
    fn edit_request(&mut self, id: String, request: RequestData) -> Result<(), ()>;
}

impl RequestServiceFacade for RequestService {
    fn add_request(&mut self, request_data: RequestData) -> String {
        let mut new_request = RequestEntity::default();
        new_request.set_request(request_data);

        "".into()
    }
    fn edit_request(&mut self, id: String, request_data: RequestData) -> Result<(), ()> {
        let req = self.requests.get_mut(&id);
        if let Some(r) = req {
            r.set_request(request_data);
        }
        Ok(())
    }
}
