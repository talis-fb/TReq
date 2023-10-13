use super::entity::{RequestData, RequestEntity};
use std::collections::HashMap;
use std::sync::Arc;

pub type RequestServiceInstance = Box<dyn RequestServiceFacade + Send>;

pub struct RequestService {
    // Possible ways to store reference to another service...
    // other_service: &'a OtherService
    // other_service: Sender<Command<ServiceB>> # Command Channel
    // other_service: Arc<SomeFacadeToServiceB> # Command Channel
    requests: HashMap<String, RequestEntity>,
}

impl RequestService {
    pub fn init() -> Self {
        Self {
            requests: HashMap::default(),
        }
    }
}

// -----------
// Facade
// -----------
use super::facade::RequestServiceFacade;

impl RequestServiceFacade for RequestService {
    fn add_request(&mut self, request_data: RequestData) -> String {
        let mut new_request = RequestEntity::default();
        new_request.update_current_request(request_data);

        let key: String = "hello".into();

        self.requests.insert(key, new_request);

        "".into()
    }
    fn edit_request(&mut self, id: String, request_data: RequestData) {
        self.requests
            .entry(id)
            .and_modify(|e| e.update_current_request(request_data));
    }

    fn delete_request(&mut self, id: String) {
        self.requests.remove(&id);
    }
    fn get_request_data(&mut self, id: String) -> Option<Arc<RequestData>> {
        Some(self.requests.get(&id)?.get_current_request())
    }
}
