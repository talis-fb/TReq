use self::entity::RequestEntity;
use std::collections::HashMap;

pub mod commands;
pub mod entity;
pub mod facade;

pub struct RequestService {
    // other_service: &'a OtherService
    requests: HashMap<String, RequestEntity>,
}

impl RequestService {
    pub fn init() -> Self {
        Self {
            requests: HashMap::default(),
        }
    }
}
