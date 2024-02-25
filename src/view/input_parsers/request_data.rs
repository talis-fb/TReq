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
                    parsers_request_items::nested_body_value,
                    parsers_request_items::non_string_body_value,
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
    use serde_json::Map;

    use super::*;
    use crate::utils::regexes;

    // Github Issue #8
    // Parse "<key>:=<null>"
    // Parse "<key>:=[-]<number>"
    // Parse "<key>:=true|false"
    // Parse "<key>:=[<null>, <string>, <number>, <boolean>, <array>, <object>]"
    // Parse "<key>:={"<key>": <null>|<string|number|boolean|array|object>"}"
    pub fn non_string_body_value(
        s: &str,
        base_request: &PartialRequestData,
    ) -> Option<PartialRequestData> {
        let mut re = regexes::request_items::non_string_body_value();
        let mut matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let value = serde_json::from_str::<Value>(value)
            .ok()
            .and_then(|v| match v {
                Value::String(_) => None,
                _ => Some(v),
            })
            .or_else(|| {
                re = regexes::request_items::enclosed_by_single_quote_value();
                matcher = re.captures(value)?;

                serde_json::from_str(matcher.name("value")?.as_str())
                    .ok()
                    .and_then(|v| match v {
                        Value::String(_) => None,
                        _ => Some(v),
                    })
            })
            .or_else(|| {
                re = regexes::request_items::enclosed_by_double_quote_value();
                matcher = re.captures(value)?;

                serde_json::from_str(matcher.name("value")?.as_str())
                    .ok()
                    .and_then(|v| match v {
                        Value::String(_) => None,
                        _ => Some(v),
                    })
            })?;

        let mut request = base_request.clone();

        request.body = match request.body {
            Some(BodyPayload::Json(serde_json::Value::Object(mut json))) => {
                json.insert(key.to_string(), value);
                BodyPayload::Json(serde_json::Value::Object(json))
            }
            _ => BodyPayload::Json(serde_json::json!({key: value})),
        }
        .into();

        Some(request)
    }

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

    pub fn nested_body_value(
        s: &str,
        base_request: &PartialRequestData,
    ) -> Option<PartialRequestData> {
        let re = regexes::request_items::body_value();
        let matcher = re.captures(s)?;

        // Key=Value
        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        // Extract the root key and sub keys from "key" input
        // ----
        // Key[SubKey1][SubKey2] => Key, [SubKey1, SubKey2]
        // ----
        let (root_key, sub_keys) = {
            let re = regexes::request_items::nested_body_keys();
            let matcher = re.captures(key)?;

            (
                matcher.name("root_key")?.as_str(),
                matcher
                    .name("sub_keys")?
                    .as_str()
                    .split(['[', ']'])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>(),
            )
        };

        let mut request = base_request.clone();

        let mut root_map: Map<String, Value> = match request.body {
            Some(BodyPayload::Json(serde_json::Value::Object(v))) => v,
            _ => Map::new(),
        };

        let root_value: &mut Value = root_map
            .entry(root_key)
            .or_insert(Value::Object(Map::new()));

        let target_value: &mut Value =
            sub_keys
                .iter()
                .fold(root_value, |map_value, key| match map_value {
                    Value::Object(map) => map
                        .entry(key.to_string())
                        .or_insert(Value::Object(Map::new())),
                    _ => {
                        *map_value = Value::Object(Map::new());
                        match map_value {
                            Value::Object(map) => map
                                .entry(key.to_string())
                                .or_insert(Value::Object(Map::new())),
                            _ => unreachable!(),
                        }
                    }
                });

        *target_value = serde_json::from_str(value)
            .ok()
            .unwrap_or(Value::String(value.to_string()));

        request.body = BodyPayload::Json(serde_json::Value::Object(root_map)).into();

        Some(request)
    }
}

#[cfg(test)]
pub mod tests_parsers_request_items {
    use super::*;

    #[test]
    fn test_non_string_body_value_with_string_only() {
        let cases = [
            r#"name:="John""#,
            r#"name:='"John"'"#,
            r#"name:=""John""#,
            r#"name:=""true"""#,
            r#"name:='"true"'"#,
        ];

        for case in cases {
            let base_request = PartialRequestData::default();

            let expected_result = None;

            assert_eq!(
                parsers_request_items::non_string_body_value(case, &base_request),
                expected_result
            );
        }
    }

    #[test]
    fn test_non_string_body_value_nested() {
        let cases = [
            (r#"hobbies:='["http", "pies"]'"#, r#"{ "hobbies": ["http", "pies"] }"#),
            (r#"hobbies:="["http", "pies"]""#, r#"{ "hobbies": ["http", "pies"] }"#),
            (r#"hobbies:=["http", "pies"]"#, r#"{ "hobbies": ["http", "pies"] }"#),
            (r#"favorite:={"tool": "HTTPie"}"#, r#"{ "favorite": { "tool": "HTTPie"} }"#),
            (r#"favorite:="{"tool": "HTTPie"}""#, r#"{ "favorite": { "tool": "HTTPie"} }"#),
            (r#"favorite:='{"tool": "HTTPie"}'"#, r#"{ "favorite": { "tool": "HTTPie"} }"#),
            (r#"complex:=[null,{},["a", false], true]"#, r#"{ "complex": [null, {}, ["a", false], true] }"#),
            (r#"complex:='{"tool": {"all":[true, 29, {"name": ["Mary", "John"]}]}}'"#, r#"{ "complex": {"tool":  {"all":[true, 29, {"name": ["Mary", "John"]}]}} }"#),
        ];

        for (input, output) in cases {
            let base_request = PartialRequestData::default();

            let expected_result = PartialRequestData::default().with_body(output.to_string());

            assert_eq!(
                parsers_request_items::non_string_body_value(input, &base_request),
                Some(expected_result)
            );
        }
    }

    #[test]
    fn test_non_string_body_value_basic() {
        let cases = [
            (r#"favorite:={}"#, r#"{ "favorite": {} }"#),
            (r#"favorite:="{}""#, r#"{ "favorite": {} }"#),
            (r#"favorite:='{}'"#, r#"{ "favorite": {} }"#),
            (r#"hobbies:=[]"#, r#"{ "hobbies": [] }"#),
            (r#"hobbies:="[]""#, r#"{ "hobbies": [] }"#),
            (r#"hobbies:='[]'"#, r#"{ "hobbies": [] }"#),
            (r#"temperature:=-28.0"#, r#"{ "temperature": -28.0 }"#),
            (r#"temperature:="27.5""#, r#"{ "temperature": 27.5 }"#),
            (r#"temperature:='-3.6'"#, r#"{ "temperature": -3.6 }"#),
            (r#"married:=true"#, r#"{ "married": true }"#),
            (r#"married:="false""#, r#"{ "married": false }"#),
            (r#"married:='true'"#, r#"{ "married": true }"#),
            (r#"worked:=null"#, r#"{ "worked": null }"#),
            (r#"worked:="null""#, r#"{ "worked": null }"#),
            (r#"worked:='null'"#, r#"{ "worked": null }"#),
        ];

        for (input, output) in cases {
            let base_request = PartialRequestData::default();

            let expected_result = PartialRequestData::default().with_body(output.to_string());

            assert_eq!(
                parsers_request_items::non_string_body_value(input, &base_request),
                Some(expected_result)
            );
        }
    }

    #[test]
    fn test_body_value_append() {
        let input = "password=123";
        let base_request =
            PartialRequestData::default().with_body(r#"{ "email": "johndoe@gmail.com" }"#);

        let expected_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123" }"#);

        assert_eq!(
            Some(expected_request),
            parsers_request_items::body_value(input, &base_request)
        )
    }

    #[test]
    fn test_body_value_append_overwrite() {
        let input = "password=123456";
        let base_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123" }"#);

        let expected_request = PartialRequestData::default()
            .with_body(r#"{ "email": "johndoe@gmail.com", "password": "123456" }"#);

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
                PartialRequestData::default().with_body(r#"{ "password": "123" }"#);
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

        let expected_request = PartialRequestData::default().with_body(r#"{ "password": "123" }"#);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::body_value(input, &base_request)
        )
    }

    #[test]
    fn test_nested_body_value_basic() {
        let cases = [
            ("user[name]=John", r#"{ "user": { "name": "John"} }"#),
            (
                "user[name][first]=John",
                r#"{ "user": { "name": { "first": "John"} } }"#,
            ),
        ];

        for (input, expected_json_output) in cases {
            let base_request = PartialRequestData::default();
            let expected_request = PartialRequestData::default().with_body(expected_json_output);
            assert_eq!(
                Some(expected_request),
                parsers_request_items::nested_body_value(input, &base_request)
            )
        }
    }

    #[test]
    fn test_nested_body_value_overwriting_fields() {
        // 1. appending in a nested object
        let input = "user[name]=John";
        let base_json = r#"{ "user": {} }"#;
        let expected_output_json = r#"{ "user": { "name": "John" } }"#;
        let base_request = PartialRequestData::default().with_body(base_json);
        let expected_request = PartialRequestData::default().with_body(expected_output_json);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::nested_body_value(input, &base_request)
        );

        // 2. overwriting existing field
        let input = "user[name]=Mary";
        let base_json = r#"{ "user": { "name": "John" } }"#;
        let expected_output_json = r#"{ "user": { "name": "Mary" } }"#;
        let base_request = PartialRequestData::default().with_body(base_json);
        let expected_request = PartialRequestData::default().with_body(expected_output_json);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::nested_body_value(input, &base_request)
        );

        // 3. appending in a nested object with another fields
        let input = "user[name][first]=John";
        let base_json = r#"{ "user": { "name": { "last": "Doe"} } }"#;
        let expected_output_json = r#"{ "user": { "name": { "first": "John", "last": "Doe"} } }"#;
        let base_request = PartialRequestData::default().with_body(base_json);
        let expected_request = PartialRequestData::default().with_body(expected_output_json);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::nested_body_value(input, &base_request)
        );

        // 4. replacing fields to object
        let input = "user[name]=Mary";
        let base_json = r#"{ "user": "a-string-to-be-replaced" }"#;
        let expected_output_json = r#"{ "user": { "name": "Mary" } }"#;
        let base_request = PartialRequestData::default().with_body(base_json);
        let expected_request = PartialRequestData::default().with_body(expected_output_json);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::nested_body_value(input, &base_request)
        );

        // 5. replacing fields to nested object
        let input = "user[name][first]=John";
        let base_json = r#"{ "user": "a-string-to-be-replaced" }"#;
        let expected_output_json = r#"{ "user": { "name": { "first": "John" } } }"#;
        let base_request = PartialRequestData::default().with_body(base_json);
        let expected_request = PartialRequestData::default().with_body(expected_output_json);
        assert_eq!(
            Some(expected_request),
            parsers_request_items::nested_body_value(input, &base_request)
        );
    }
}
