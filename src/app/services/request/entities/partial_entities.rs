use std::collections::HashMap;
use std::str::FromStr;

use serde::Serialize;

use super::methods::METHODS;
use super::requests::{BodyPayload, RequestData};
use super::url::{Url, UrlInfo};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PartialRequestData {
    pub url: Option<Url>,
    pub method: Option<METHODS>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<BodyPayload>,
}

impl PartialRequestData {
    pub fn with_url(mut self, value: impl Into<String>) -> Self {
        let value: String = value.into();
        let url = match UrlInfo::from_str(&value) {
            Ok(url) => Url::ValidatedUrl(url),
            Err(_) => Url::Raw(value),
        };
        self.url = Some(url);
        self
    }

    pub fn with_method(mut self, value: METHODS) -> Self {
        self.method = Some(value);
        self
    }

    pub fn with_body(mut self, value: impl Into<BodyPayload>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn with_headers(mut self, values: impl Into<HashMap<String, String>>) -> Self {
        self.headers = Some(values.into());
        self
    }
}

impl From<RequestData> for PartialRequestData {
    fn from(value: RequestData) -> Self {
        Self {
            url: Some(value.url),
            method: Some(value.method),
            headers: Some(value.headers),
            body: Some(value.body),
        }
    }
}

impl PartialRequestData {
    pub fn to_request_data(self) -> RequestData {
        RequestData::default()
            .with_url(
                self.url
                    .expect("Url is required to define a Request Data")
                    .to_string(),
            )
            .with_method(
                self.method
                    .expect("METHOD is required to define a Request Data"),
            )
            .with_headers(self.headers.unwrap_or_default())
            .with_body_payload(self.body.unwrap_or_default())
    }
}
