use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum METHODS {
    #[default]
    GET,
    DELETE,
    HEAD,
    PATCH,
    POST,
    PUT,
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
        self.url = value.into();
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
    pub fn with_headers<const N: usize>(mut self, values: [(String, String); N]) -> Self {
        self.headers = HashMap::from(values);
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
