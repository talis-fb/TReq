use std::sync::Arc;

use super::entities::requests::RequestData;
use crate::utils::uuid::UUID;

pub trait RequestServiceFacade: Send {
    fn add_request(&mut self, request: RequestData) -> UUID;
    fn edit_request(&mut self, id: UUID, request: RequestData);
    fn delete_request(&mut self, id: UUID);
    fn get_request_data(&self, id: UUID) -> Option<Arc<RequestData>>;
    fn undo_request_data(&mut self, id: UUID);
    fn redo_request_data(&mut self, id: UUID);
}
