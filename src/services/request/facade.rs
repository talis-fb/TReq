use std::sync::Arc;

use super::entity::{RequestData, RequestEntity};

// pub type ResultAtomicRequestServiceOperation<T> = Result<T, ErrAtomic<Box<dyn RequestServiceFacade>>>;
// use ResultAtomicRequestServiceOperation as ResultAtomic;

// HERE, each method that return a result is ATOMIC.
// If it returns a error the same method should not to change
// anything about the service initial state.
// Or the result is right and it changes or nothing happens.

pub trait RequestServiceFacade {
    fn add_request(&mut self, request: RequestData) -> String;
    fn edit_request(&mut self, id: String, request: RequestData);
    fn delete_request(&mut self, id: String);
    fn get_request_data(&mut self, id: String) -> Option<Arc<RequestData>>;
}
