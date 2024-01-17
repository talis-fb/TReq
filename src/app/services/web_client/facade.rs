use super::entities::Response;
use crate::app::services::request::entities::RequestData;
use crate::utils::uuid::UUID;

pub trait WebClientFacade: Send {
    fn submit(&mut self, request: RequestData) -> UUID;
    fn get_task_request(
        &mut self,
        id: UUID,
    ) -> Option<tokio::task::JoinHandle<Result<Response, String>>>;
}
