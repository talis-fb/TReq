use std::collections::HashMap;
use std::sync::Arc;

use super::entity::{RequestData, RequestEntity};
use crate::utils::uuid::UUID;

pub type RequestServiceInstance = Box<dyn RequestServiceFacade + Send>;

pub struct RequestService {
    // Possible ways to store reference to another service...
    // other_service: &'a OtherService
    // other_service: Sender<Command<ServiceB>> # Command Channel
    // other_service: Arc<SomeFacadeToServiceB> # Command Channel
    requests: HashMap<UUID, RequestEntity>,
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
    fn add_request(&mut self, request_data: RequestData) -> UUID {
        let mut new_request = RequestEntity::default();
        new_request.update_current_request(request_data);
        let id = UUID::new_random();
        self.requests.insert(id.clone(), new_request);
        id
    }

    fn edit_request(&mut self, id: UUID, request_data: RequestData) {
        self.requests
            .entry(id)
            .and_modify(|e| e.update_current_request(request_data));
    }

    fn delete_request(&mut self, id: UUID) {
        self.requests.remove(&id);
    }

    fn get_request_data(&self, id: UUID) -> Option<Arc<RequestData>> {
        Some(self.requests.get(&id)?.get_current_request())
    }

    fn rollback_request_data(&mut self, id: UUID) -> () {
        self.requests.entry(id).and_modify(|e| e.rollback());
    }
}
