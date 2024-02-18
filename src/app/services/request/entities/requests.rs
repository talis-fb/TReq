use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::partial_entities::PartialRequestData;
use super::url::{Url, UrlInfo};
use crate::app::services::request::entities::methods::METHODS;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestData {
    pub url: Url,
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: BodyPayload,
}

impl RequestData {
    pub fn with_url(mut self, value: impl Into<String>) -> Self {
        let value: String = value.into();
        self.url = match UrlInfo::from_str(&value) {
            Ok(url) => Url::ValidatedUrl(url),
            Err(_) => Url::Raw(value),
        };
        self
    }
    pub fn with_body(mut self, value: impl Into<BodyPayload>) -> Self {
        self.body = value.into();
        self
    }
    pub fn with_body_payload(mut self, value: BodyPayload) -> Self {
        self.body = value;
        self
    }
    pub fn with_method(mut self, value: METHODS) -> Self {
        self.method = value;
        self
    }
    pub fn with_headers(mut self, values: impl Into<HashMap<String, String>>) -> Self {
        self.headers = values.into();
        self
    }

    pub fn merge(mut self, other: PartialRequestData) -> Self {
        // Method
        self.method = other.method.unwrap_or(self.method);

        // Url
        if let Some(other_url) = other.url {
            match (self.url, other_url) {
                (Url::ValidatedUrl(current_url_info), Url::ValidatedUrl(other_url_info)) => {
                    let new_url_info = current_url_info.be_overwrite_by(other_url_info);
                    self.url = Url::ValidatedUrl(new_url_info);
                }
                (_, other_url) => {
                    self.url = other_url;
                }
            }
        }

        // Headers
        self.headers.extend(other.headers.unwrap_or_default());

        // Body
        if let Some(other_body) = other.body {
            match (self.body, other_body) {
                (
                    BodyPayload::Json(Value::Object(mut current_map_json)),
                    BodyPayload::Json(Value::Object(other_map_json)),
                ) => {
                    current_map_json.extend(other_map_json);
                    self.body = BodyPayload::Json(serde_json::Value::Object(current_map_json));
                }
                (_, other_body) => {
                    self.body = other_body;
                }
            };
        }

        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyPayload {
    Raw(String),
    Json(serde_json::Value),
}

impl Default for BodyPayload {
    fn default() -> Self {
        BodyPayload::Raw(String::default())
    }
}

impl BodyPayload {
    pub fn from_str(value: &str) -> Self {
        match serde_json::from_str::<Value>(value) {
            Ok(value) => BodyPayload::Json(value),
            Err(_) => BodyPayload::Raw(value.to_string()),
        }
    }
}

impl ToString for BodyPayload {
    fn to_string(&self) -> String {
        match self {
            BodyPayload::Raw(value) => value.to_string(),
            BodyPayload::Json(value) => value.to_string(),
        }
    }
}

impl From<String> for BodyPayload {
    fn from(value: String) -> Self {
        BodyPayload::from_str(&value)
    }
}

#[derive(Default)]
pub struct RequestEntity {
    current_request: Box<NodeHistoryRequest>,
}

impl RequestEntity {
    pub fn get_current_request(&self) -> Arc<RequestData> {
        self.current_request.data.clone()
    }

    pub fn update_current_request(&mut self, request_data: RequestData) {
        let new_node = Box::from(NodeHistoryRequest::from(request_data));
        let last_state = std::mem::replace(&mut self.current_request, new_node);
        self.current_request.previous = Some(last_state);
    }

    pub fn undo(&mut self) {
        if let Some(previous_req_node) = self.current_request.previous.take() {
            let last_state = std::mem::replace(&mut self.current_request, previous_req_node);
            self.current_request.next = Some(last_state);
        }
    }

    pub fn redo(&mut self) {
        if let Some(next_req_node) = self.current_request.next.take() {
            let last_state = std::mem::replace(&mut self.current_request, next_req_node);
            self.current_request.previous = Some(last_state);
        }
    }
}

impl From<RequestData> for RequestEntity {
    fn from(value: RequestData) -> Self {
        Self {
            current_request: Box::from(NodeHistoryRequest::from(value)),
        }
    }
}

#[derive(Default)]
struct NodeHistoryRequest {
    pub data: Arc<RequestData>,
    pub previous: Option<Box<NodeHistoryRequest>>,
    pub next: Option<Box<NodeHistoryRequest>>,
}
impl From<RequestData> for NodeHistoryRequest {
    fn from(value: RequestData) -> Self {
        let data = Arc::new(value);
        Self {
            data,
            previous: None,
            next: None,
        }
    }
}
