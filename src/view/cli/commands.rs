use std::collections::HashMap;

use crate::app::services::request::entity::{RequestData, METHODS};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct OptionalRequestData {
    pub url: Option<String>,
    pub method: Option<METHODS>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl From<RequestData> for OptionalRequestData {
    fn from(value: RequestData) -> Self {
        Self {
            url: Some(value.url),
            method: Some(value.method),
            headers: Some(value.headers),
            body: Some(value.body),
        }
    }
}

impl OptionalRequestData {
    pub fn to_request_data(self) -> RequestData {
        RequestData::default()
            .with_url(self.url.unwrap_or_default())
            .with_method(self.method.unwrap_or_default())
            .with_headers(self.headers.unwrap_or_default())
            .with_body(self.body.unwrap_or_default())
    }

    pub fn merge_with(self, other: RequestData) -> RequestData {
        RequestData::default()
            .with_url(self.url.unwrap_or(other.url))
            .with_method(self.method.unwrap_or(other.method))
            .with_headers(self.headers.unwrap_or(other.headers))
            .with_body(self.body.unwrap_or(other.body))
    }
}

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
        request: OptionalRequestData,
        request_name: String,
    },

    RemoveSavedRequest {
        request_name: String,
    },
    RenameSavedRequest {
        request_name: String,
        new_name: String,
    },
}
