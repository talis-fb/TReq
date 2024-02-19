use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;

use crate::app::services::request::entities::methods::METHODS;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::app::services::request::entities::requests::BodyPayload;
use crate::app::services::request::entities::url::{Url, UrlInfo};
use crate::view::input::cli_input::{CliCommandChoice, CliInput, RequestBuildingOptions};

pub fn parse_inputs_to_request_data(input: &CliInput) -> Result<PartialRequestData> {
    // Optional params like '--url', '--method' or '--raw'
    let base_request = {
        let RequestBuildingOptions {
            raw_body,
            url_manual,
            method_manual,
            ..
        } = &input.request_input;

        let mut req = PartialRequestData::default();
        req.method = *method_manual;
        req.url = url_manual.as_ref().map(|value| Url::from_str(value));
        req.body = raw_body.as_ref().map(|v| BodyPayload::from_str(v));
        req
    };

    // Request data from 'CliCommandChoice'
    let base_request = match input.choice {
        CliCommandChoice::BasicRequest { method, ref url } => {
            base_request.with_method(method).with_url(url)
        }
        CliCommandChoice::DefaultBasicRequest { ref url } => {
            base_request.with_method(METHODS::GET).with_url(url)
        }
        _ => base_request,
    };

    // From request items
    let base_request =
        input
            .request_input
            .request_items
            .iter()
            .fold(base_request, |req_data, item| {
                [
                    parsers_request_items::query_param_value,
                    parsers_request_items::body_value,
                    parsers_request_items::header_value,
                ]
                .into_iter()
                .find_map(|parser| parser(item.as_ref(), &req_data))
                .unwrap_or(req_data)
            });

    Ok(base_request)
}

mod parsers_request_items {
    use super::*;
    use crate::app::services::request::entities::requests::BodyPayload;
    use crate::utils::regexes;

    pub fn body_value(s: &str, base_request: &PartialRequestData) -> Option<PartialRequestData> {
        let re = regexes::request_items::body_value();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let mut request = base_request.clone();

        request.body = match request.body {
            Some(BodyPayload::Json(serde_json::Value::Object(mut json))) => {
                json.insert(key.to_string(), Value::String(value.to_string()));
                BodyPayload::Json(serde_json::Value::Object(json))
            }
            _ => BodyPayload::Json(serde_json::json!({key: value})),
        }
        .into();

        Some(request)
    }

    pub fn header_value(s: &str, base_request: &PartialRequestData) -> Option<PartialRequestData> {
        let re = regexes::request_items::header_value();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let mut request = base_request.clone();

        request
            .headers
            .get_or_insert(HashMap::new())
            .insert(key.to_string(), value.to_string());

        Some(request)
    }

    pub fn query_param_value(
        s: &str,
        base_request: &PartialRequestData,
    ) -> Option<PartialRequestData> {
        let re = regexes::request_items::query_param_value();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        // TODO: RETURN THIS TO USER
        // In this case the validation on URL is already made and is not possible to manipulate it
        // to insert a query_param, because was not possible to create the UrlInfo using given input
        if let Some(Url::Raw(_)) = base_request.url.as_ref() {
            return None;
        }

        let mut request = base_request.clone();
        request.url = request.url.or(Some(Url::ValidatedUrl(UrlInfo::default())));

        if let Some(Url::ValidatedUrl(url_data)) = request.url.as_mut() {
            url_data
                .query_params
                .push((key.to_string(), value.to_string()));
        }

        Some(request)
    }
}

#[cfg(test)]
pub mod tests_parsers_request_items {
    use super::*;

    #[test]
    fn test_body_value_append() {
        let input = "password=123";
        let base_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com" }"#.to_string());

        let expected_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123" }"#.to_string());

        assert_eq!(
            Some(expected_request),
            parsers_request_items::body_value(input, &base_request)
        )
    }

    #[test]
    fn test_body_value_append_overwrite() {
        let input = "password=123456";
        let base_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123" }"#.to_string());

        let expected_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123456" }"#.to_string());

        assert_eq!(
            Some(expected_request),
            parsers_request_items::body_value(input, &base_request)
        )
    }

    #[test]
    fn test_body_value_with_base_body_as_raw() {
        let input = "password=123";

        let base_requests_body = ["anything", r#"["element1", "element2"]"#, "10", ""];
        let base_requests = base_requests_body
            .map(|body| PartialRequestData::default().with_body(body.to_string()));

        for request in base_requests {
            let expected_request =
                PartialRequestData::default().with_body(r#"{ "password": "123" }"#.to_string());
            assert_eq!(
                Some(expected_request),
                parsers_request_items::body_value(input, &request)
            )
        }
    }
    #[test]

    fn test_body_value_with_base_body_none() {
        let input = "password=123";
        let base_request = PartialRequestData::default();

        let expected_request =
            PartialRequestData::default().with_body(r#"{ "password": "123" }"#.to_string());
        assert_eq!(
            Some(expected_request),
            parsers_request_items::body_value(input, &base_request)
        )
    }
}
