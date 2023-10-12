use std::sync::Arc;

use super::entity::{RequestData, RequestEntity};
use super::RequestService;

// pub type ResultAtomicRequestServiceOperation<T> = Result<T, ErrAtomic<Box<dyn RequestServiceFacade>>>;
// use ResultAtomicRequestServiceOperation as ResultAtomic;

// HERE, each method that return a result is ATOMIC.
// If it returns a error the same method should not to change
// anything about the service initial state.
// Or the result is right and it changes or nothing happens.

pub trait RequestServiceFacade  {
    fn add_request(&mut self, request: RequestData) -> String;
    fn edit_request(&mut self, id: String, request: RequestData);
    fn delete_request(&mut self, id: String);
    fn get_request_data(&mut self, id: String) -> Option<Arc<RequestData>>;
}

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
