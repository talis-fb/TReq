use std::collections::HashMap;

use super::facade::WebClientFacade;
use super::repository_client::{HttpClientRepository, TaskRunningRequest};
use crate::app::services::request::entity::{RequestData, METHODS};
use crate::utils::uuid::UUID;

pub type WebClientInstance = Box<dyn WebClientFacade + Send>;

pub struct WebClient {
    running_requests: HashMap<UUID, TaskRunningRequest>,
    pub http_client: Box<dyn HttpClientRepository>,
}

impl WebClient {
    pub fn init(repository: impl HttpClientRepository + 'static) -> Self {
        Self {
            http_client: Box::new(repository),
            running_requests: HashMap::new(),
        }
    }
}

impl WebClientFacade for WebClient {
    fn submit(&mut self, request: RequestData) -> UUID {
        let RequestData {
            url, headers, body, ..
        } = request;

        let method_to_call = match request.method {
            METHODS::GET => self.http_client.call_get(url, headers),
            METHODS::POST => self.http_client.call_post(url, headers, body),
            METHODS::PUT => self.http_client.call_put(url, headers, body),
            METHODS::PATCH => self.http_client.call_patch(url, headers, body),
            METHODS::HEAD => self.http_client.call_head(url, headers, body),
            METHODS::DELETE => self.http_client.call_delete(url, headers, body),
        };

        let id_task = UUID::new_random();

        self.running_requests
            .insert(id_task.clone(), method_to_call);

        id_task
    }

    fn get_task_request(&mut self, id: UUID) -> Option<TaskRunningRequest> {
        let task = self.running_requests.remove(&id)?;
        Some(task)
    }
}
