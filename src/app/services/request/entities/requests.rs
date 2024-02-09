use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::url::{Url, UrlInfo};
use crate::app::services::request::entities::methods::METHODS;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestData {
    pub url: Url,
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: String,
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
    pub fn with_body(mut self, value: impl Into<String>) -> Self {
        self.body = value.into();
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

// Used to
// #[derive(Default, Clone, Debug, PartialEq, Eq, Serialize)]
// pub struct PartialRequestData {
//     pub url: Option<Url>,
//     pub method: Option<METHODS>,
//     pub headers: Option<HashMap<String, String>>,
//     pub body: Option<String>,
// }
//
// impl PartialRequestData {
//     pub fn with_url(mut self, value: impl Into<String>) -> Self {
//         let value: String = value.into();
//         let url = match UrlDatas::from_str(&value) {
//             Ok(url) => Url::ValidatedUrl(url),
//             Err(_) => Url::Raw(value),
//         };
//         self.url = Some(url);
//         self
//     }
//
//     pub fn with_method(mut self, value: METHODS) -> Self {
//         self.method = Some(value);
//         self
//     }
// }
//
// impl From<RequestData> for PartialRequestData {
//     fn from(value: RequestData) -> Self {
//         Self {
//             url: Some(value.url),
//             method: Some(value.method),
//             headers: Some(value.headers),
//             body: Some(value.body),
//         }
//     }
// }
//
// impl PartialRequestData {
//     pub fn to_request_data(self) -> RequestData {
//         RequestData::default()
//             .with_url(
//                 self.url
//                     .expect("Url is required to define a Request Data")
//                     .to_string(),
//             )
//             .with_method(
//                 self.method
//                     .expect("METHOD is required to define a Request Data"),
//             )
//             .with_headers(self.headers.unwrap_or_default())
//             .with_body(self.body.unwrap_or_default())
//     }
//
//     pub fn merge_with(self, other: RequestData) -> RequestData {
//         RequestData::default()
//             .with_url(self.url.unwrap_or(other.url).to_string())
//             .with_method(self.method.unwrap_or(other.method))
//             .with_headers(self.headers.unwrap_or(other.headers))
//             .with_body(self.body.unwrap_or(other.body))
//     }
// }
