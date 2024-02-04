pub mod reqwest;

use mockall::automock;
use mockall::predicate::*;
use tokio::task::JoinHandle as TokioTask;

use super::entities::Response;
use crate::app::services::request::entities::requests::RequestData;

pub type TaskRunningRequest = TokioTask<anyhow::Result<Response>>;

// -------------------------------------------------------------------------------------------------------------------
// TODO: Make this 'automock' enabled only in test mode
//  and also move the dependency definition to dev-dependencies,
//  but doing it now breaks web_client's integration tests. At importing of Mock HttpRepository
// -------------------------------------------------------------------------------------------------------------------

#[automock]
pub trait HttpClientRepository: Send {
    fn submit_request(&self, request: RequestData) -> TaskRunningRequest;
}
