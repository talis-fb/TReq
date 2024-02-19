use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

use super::super::entities::{Response, ResponseStage};
use super::{HttpClientRepository, TaskRunningRequest};
use crate::app::services::request::entities::methods::METHODS;
use crate::app::services::request::entities::requests::RequestData;

#[derive(Default)]
pub struct ReqwestClientRepository;

impl HttpClientRepository for ReqwestClientRepository {
    fn submit_request(&self, request: RequestData) -> TaskRunningRequest {
        tokio::task::spawn(async move {
            let url = request.url.to_string();
            let headers = request.headers;
            let method = request.method;

            let client = Client::new();
            let client = match method {
                METHODS::GET => client.get(url),
                METHODS::POST => client.post(url),
                METHODS::DELETE => client.delete(url),
                METHODS::PATCH => client.patch(url),
                METHODS::PUT => client.put(url),
                METHODS::HEAD => client.head(url),
            };

            let mut client = client.headers(ReqwestClientRepository::create_header_map(headers));

            if method != METHODS::GET {
                let body = request.body;
                client = client.body(body.to_string());
            }

            let now = tokio::time::Instant::now();

            let response = client.send().await?;

            let response_time_ms = now.elapsed().as_millis() as u64;

            ReqwestClientRepository::convert_to_app_response(response, response_time_ms).await
        })
    }
}

impl ReqwestClientRepository {
    fn create_header_map(headers: HashMap<String, String>) -> HeaderMap {
        let mut headers_reqwest = HeaderMap::new();

        for (key, value) in headers.into_iter() {
            headers_reqwest.insert(
                HeaderName::from_str(&key).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        headers_reqwest
    }

    async fn convert_to_app_response(response: reqwest::Response, response_time_ms: u64) -> anyhow::Result<Response> {
        let status: i32 = response.status().as_u16().into();
        let mut headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(key, value)| {
                (
                    key.as_str().to_string(),
                    value.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect();

        headers.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let body = response.text().await?;

        Ok(Response {
            status,
            body,
            response_time_ms,
            headers,
            stage: ResponseStage::Finished,
        })
    }
}
