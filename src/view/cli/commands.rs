use crate::app::services::request::entity::{OptionalRequestData, RequestData};

#[derive(Debug, PartialEq, Eq)]
pub enum CliCommand {
    SubmitRequest {
        request: RequestData,
    },

    SubmitSavedRequest {
        request_name: String,
    },
    SubmitSavedRequestWithAdditionalData {
        request_name: String,
        request_data: OptionalRequestData,
    },

    SaveRequest {
        request_name: String,
        request: OptionalRequestData,
    },

    RemoveSavedRequest {
        request_name: String,
    },

    RenameSavedRequest {
        request_name: String,
        new_name: String,
    },
}
