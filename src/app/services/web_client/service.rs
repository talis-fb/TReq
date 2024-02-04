use tokio::task::JoinHandle;

use super::entities::Response;
use super::facade::WebClientFacade;
use super::repository_client::HttpClientRepository;
use crate::app::services::request::entities::methods::METHODS;
use crate::app::services::request::entities::requests::RequestData;

pub type WebClientInstance = Box<dyn WebClientFacade>;

pub struct WebClient {
    pub http_client: Box<dyn HttpClientRepository>,
}

impl WebClient {
    pub fn init(repository: impl HttpClientRepository + 'static) -> Self {
        Self {
            http_client: Box::new(repository),
        }
    }
}

impl WebClientFacade for WebClient {
    fn submit_async(&mut self, request: RequestData) -> JoinHandle<Result<Response, String>> {
        let RequestData {
            url, headers, body, ..
        } = request;

        match request.method {
            METHODS::GET => self.http_client.call_get(url.to_string(), headers),
            METHODS::POST => self.http_client.call_post(url.to_string(), headers, body),
            METHODS::PUT => self.http_client.call_put(url.to_string(), headers, body),
            METHODS::PATCH => self.http_client.call_patch(url.to_string(), headers, body),
            METHODS::HEAD => self.http_client.call_head(url.to_string(), headers, body),
            METHODS::DELETE => self.http_client.call_delete(url.to_string(), headers, body),
        }
    }
}
