use std::collections::HashMap;
use std::sync::Arc;

use super::entities::{RequestData, RequestEntity};
use crate::utils::uuid::UUID;

pub type RequestServiceInstance = Box<dyn RequestServiceFacade>;

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

// --------------
// Impl facade
// --------------
use super::facade::RequestServiceFacade;

impl RequestServiceFacade for RequestService {
    fn add_request(&mut self, request_data: RequestData) -> UUID {
        let new_request = RequestEntity::from(request_data);
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

    fn undo_request_data(&mut self, id: UUID) {
        self.requests.entry(id).and_modify(|e| e.undo());
    }

    fn redo_request_data(&mut self, id: UUID) {
        self.requests.entry(id).and_modify(|e| e.redo());
    }
}
