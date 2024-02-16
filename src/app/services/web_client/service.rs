use super::facade::WebClientFacade;
use super::repository_client::{HttpClientRepository, TaskRunningRequest};
use crate::app::services::request::entities::requests::RequestData;
use crate::app::services::request::entities::url::Url;

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
    fn submit_async(&mut self, mut request: RequestData) -> TaskRunningRequest {
        if let Url::ValidatedUrl(url) = &mut request.url {
            url.protocol.get_or_insert("http".to_string());
        }

        self.http_client.submit_request(request)
    }
}
