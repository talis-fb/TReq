use tokio::task::JoinHandle;

use super::entities::Response;
use crate::app::services::request::entities::requests::RequestData;

pub trait WebClientFacade: Send {
    fn submit_async(&mut self, request: RequestData) -> JoinHandle<anyhow::Result<Response>>;
}
