pub mod reqwest;

use std::collections::HashMap;

use mockall::automock;
use mockall::predicate::*;
use tokio::task::JoinHandle as TokioTask;

use super::entities::Response;

pub type TaskRunningRequest = TokioTask<Result<Response, String>>;

// -------------------------------------------------------------------------------------------------------------------
// TODO: Make this 'automock' enabled only in test mode
//  and also move the dependency definition to dev-dependencies,
//  but doing it now breaks web_client's integration tests. At importing of Mock HttpRepository
// -------------------------------------------------------------------------------------------------------------------

#[automock]
pub trait HttpClientRepository: Send {
    fn call_get(&self, url: String, headers: HashMap<String, String>) -> TaskRunningRequest;
    fn call_post(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> TaskRunningRequest;
    fn call_delete(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> TaskRunningRequest;
    fn call_patch(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> TaskRunningRequest;
    fn call_put(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> TaskRunningRequest;
    fn call_head(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> TaskRunningRequest;
}
