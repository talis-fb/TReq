use std::{
    collections::{HashMap, LinkedList},
    sync::Arc,
};

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

#[derive(Default, Clone, Debug)]
pub struct RequestData {
    pub url: String,
    pub name: String,
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Default)]
pub struct RequestEntity {
    current_request: Arc<RequestData>,
    history: LinkedList<Arc<RequestData>>,
}

impl RequestEntity {
    pub fn get_current_request(&self) -> Arc<RequestData> {
        self.current_request.clone()
    }

    pub fn update_current_request(&mut self, request_data: RequestData) -> () {
        let new_request_data = Arc::new(request_data);
        let old_request_data = std::mem::replace(&mut self.current_request, new_request_data);
        self.history.push_back(old_request_data);
    }

    pub fn rollback(&mut self) {
        if let Some(state) = self.history.pop_back() {
            self.current_request = state;
        }
    }
}
