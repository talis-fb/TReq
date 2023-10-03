use std::collections::HashMap;
use self::entity::RequestEntity;

pub mod commands;
pub mod entity;
pub mod facade;
pub mod runner;

pub struct RequestService {
    // Possible ways to store reference to another service...
    // other_service: &'a OtherService
    // other_service: Sender<Command<ServiceB>> # Command Channel
    // other_service: Arc<SomeFacadeToServiceB> # Command Channel
    requests: HashMap<String, RequestEntity>,
}

impl RequestService {
    pub fn init() -> Self {
        Self {
            requests: HashMap::default(),
        }
    }
}
