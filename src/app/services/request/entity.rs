use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum METHODS {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
}
impl ToString for METHODS {
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}
impl METHODS {
    pub fn as_str(&self) -> &'static str {
        match self {
            METHODS::GET => "GET",
            METHODS::POST => "POST",
            METHODS::PUT => "PUT",
            METHODS::DELETE => "DELETE",
            METHODS::HEAD => "HEAD",
            METHODS::PATCH => "PATCH",
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct RequestData {
    pub url: String,
    pub name: String,
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl RequestData {
    pub fn with_url(mut self, value: impl Into<String>) -> Self {
        let mut value: String = value.into();

        if !(value.starts_with("http://") || value.starts_with("https://")) {
            value = format!("{}{}", "http://", value);
        }

        self.url = value;
        self
    }
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
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

#[cfg(test)]
mod tests {
    use super::RequestData;

    #[test]
    fn test_url_with_protocol_default() {
        let start_with_https = RequestData::default().with_url("https://google.com");
        assert_eq!("https://google.com", start_with_https.url);

        let start_with_http = RequestData::default().with_url("http://duck.com");
        assert_eq!("http://duck.com", start_with_http.url);

        let start_without_protocol = RequestData::default().with_url("duck.com");
        assert_eq!("http://duck.com", start_without_protocol.url);
    }
}
