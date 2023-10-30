use std::collections::HashMap;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResponseStage {
    #[default]
    Empty,

    Waiting,
    Finished,

    Cancelled,
    InternalError,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub status: i32,
    pub response_time: u64,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub stage: ResponseStage,
}
