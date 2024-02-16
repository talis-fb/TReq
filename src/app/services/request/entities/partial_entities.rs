use std::collections::HashMap;
use std::str::FromStr;

use serde::Serialize;

use super::methods::METHODS;
use super::requests::RequestData;
use super::url::{Url, UrlInfo};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PartialRequestData {
    pub url: Option<Url>,
    pub method: Option<METHODS>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
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
            .with_body(self.body.unwrap_or_default())
    }

    pub fn merge_with(self, other: RequestData) -> RequestData {
        let mut final_request = RequestData::default()
            .with_method(self.method.unwrap_or(other.method))
            .with_headers(self.headers.unwrap_or(other.headers))
            .with_body(self.body.unwrap_or(other.body));

        match (self.url, other.url) {
            (Some(Url::ValidatedUrl(url)), Url::ValidatedUrl(other)) => {
                final_request.url = Url::ValidatedUrl(other.be_overwrite_by(url));
            }
            (Some(Url::Raw(raw_url)), _) => {
                final_request.url = Url::Raw(raw_url);
            }
            (_, other) => {
                final_request.url = other;
            }
        };

        final_request
    }
}
