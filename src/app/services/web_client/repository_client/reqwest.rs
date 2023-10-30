use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

use super::super::entity::{Response, ResponseStage};
use super::{HttpClientRepository, ResponseType};

#[derive(Default)]
pub struct ReqwestClientRepository;

impl ReqwestClientRepository {
    fn create_header_map(map: HashMap<String, String>) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (key, value) in map.into_iter() {
            headers.insert(
                HeaderName::from_str(&key).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        headers
    }

    async fn convert_to_app_response(response: reqwest::Response) -> Result<Response, String> {
        let status: i32 = response.status().as_u16().into();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(key, value)| {
                (
                    key.as_str().to_string(),
                    value.to_str().unwrap().to_string(),
                )
            })
            .collect();

        let body = response.text().await.map_err(|e| e.to_string())?;

        Ok(Response {
            status,
            body,
            response_time: 1,
            headers,
            stage: ResponseStage::Finished,
        })
    }
}

impl HttpClientRepository for ReqwestClientRepository {
    fn call_get(&self, url: String, _headers: HashMap<String, String>) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client.get(url).send().await.map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }

    fn call_post(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client
                .post(url)
                .body(body)
                .headers(ReqwestClientRepository::create_header_map(headers))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }

    fn call_delete(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client
                .delete(url)
                .body(body)
                .headers(ReqwestClientRepository::create_header_map(headers))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }

    fn call_patch(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client
                .patch(url)
                .body(body)
                .headers(ReqwestClientRepository::create_header_map(headers))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }

    fn call_put(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client
                .put(url)
                .body(body)
                .headers(ReqwestClientRepository::create_header_map(headers))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }

    fn call_head(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> ResponseType {
        tokio::task::spawn(async move {
            let client = Client::new();
            let response = client
                .head(url)
                .body(body)
                .headers(ReqwestClientRepository::create_header_map(headers))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            ReqwestClientRepository::convert_to_app_response(response).await
        })
    }
}
