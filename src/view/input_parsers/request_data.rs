use std::collections::HashMap;

use anyhow::Result;
use serde_json::{Map, Value};

use crate::app::services::request::entities::methods::METHODS;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
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
        req.body = raw_body.clone();
        req.method = *method_manual;
        req.url = url_manual.as_ref().map(|value| Url::from_str(value));
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
    use crate::utils::regexes;

    pub fn body_value(s: &str, base_request: &PartialRequestData) -> Option<PartialRequestData> {
        let re = regexes::request_items::body_value();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let original_body = base_request.body.as_deref().unwrap_or("{}");

        let mut request = base_request.clone();

        request.body = {
            let mut json =
                serde_json::from_str::<Map<String, Value>>(original_body).unwrap_or_default();
            json.insert(key.to_string(), Value::String(value.to_string()));
            serde_json::to_string(&json).unwrap_or_default().into()
        };

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
