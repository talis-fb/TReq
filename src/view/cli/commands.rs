use crate::app::services::request::entity::RequestData;

#[derive(Debug, PartialEq, Eq)]
pub enum CliCommand {
    /// A basic GET request
    BasicRequest { request: RequestData },
}
