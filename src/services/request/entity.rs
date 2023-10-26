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
        let request_data = Arc::new(request_data);
        self.history.push_back(request_data.clone());
        self.current_request = request_data.clone();
    }
}
