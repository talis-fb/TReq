use std::sync::Arc;

use super::entity::RequestData;
use crate::utils::uuid::UUID;

// pub type ResultAtomicRequestServiceOperation<T> = Result<T, ErrAtomic<Box<dyn RequestServiceFacade>>>;
// use ResultAtomicRequestServiceOperation as ResultAtomic;

// HERE, each method that return a result is ATOMIC.
// If it returns a error the same method should not to change
// anything about the service initial state.
// Or the result is right and it changes or nothing happens.

pub trait RequestServiceFacade {
    fn add_request(&mut self, request: RequestData) -> UUID;
    fn edit_request(&mut self, id: UUID, request: RequestData);
    fn delete_request(&mut self, id: UUID);
    fn get_request_data(&self, id: UUID) -> Option<Arc<RequestData>>;
    fn undo_request_data(&mut self, id: UUID);
    fn redo_request_data(&mut self, id: UUID);
}
