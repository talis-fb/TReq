use std::{
    collections::{HashMap, LinkedList},
    rc::Rc,
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
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Default)]
pub struct RequestEntity {
    pub name: String,
    current_request: Rc<RequestData>,
    history: LinkedList<Rc<RequestData>>,
}

impl RequestEntity {
    pub fn update_current_request(&mut self, request_data: RequestData) -> () {
        let request_data = Rc::new(request_data);
        self.history.push_back(request_data.clone());
        self.current_request = request_data.clone();
    }
}
