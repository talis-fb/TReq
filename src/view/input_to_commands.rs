use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;
use serde_json::{Map, Value};

use crate::app::services::request::entities::requests::OptionalRequestData;
use crate::view::commands::ViewCommandChoice;
use crate::view::input::cli_input::CliInput;

pub fn map_input_to_commands(input: CliInput) -> Result<Vec<ViewCommandChoice>> {
    // -------------------------------------
    // The builded requests by inputs
    // -------------------------------------
    let base_request: Option<OptionalRequestData> = {
        match input {
            CliInput::DefaultBasicRequest {
                url, ref request_items, ..
            } => {
                let base_url = url.to_string();
                let base_request = OptionalRequestData::default().with_url(base_url);
                let request = parser_request_items_to_data(base_request, &request_items);
                Some(request)
            }
            CliInput::BasicRequest {
                url,
                method,
                ref request_items,
                ..
            } => {
                let base_url = url.to_string();
                let base_request = OptionalRequestData::default()
                    .with_url(base_url)
                    .with_method(method);
                let request = parser_request_items_to_data(base_request, &request_items);
                Some(request)
            }
            CliInput::Run {
                url_manual,
                method_manual,
                ref request_items,
                ..
            }
            | CliInput::Edit {
                url_manual,
                method_manual,
                ref request_items,
                ..
            } => {
                let base_request = {
                    let mut req = OptionalRequestData::default();
                    if let Some(url) = url_manual {
                        req = req.with_url(url.to_string());
                    }
                    if let Some(method) = method_manual {
                        req = req.with_method(method);
                    }
                    req
                };

                let request = parser_request_items_to_data(base_request, &request_items);
                Some(request)
            }
            _ => None,
        }
    };

    // -----------------------------------------------------
    // Commands to run before the main commands wished
    //   Theses commands are defined by optional flags
    //   like '--save-as' or '--save'
    // -----------------------------------------------------
    let pre_commands: Vec<ViewCommandChoice> = {
        let mut save_as = match input {
            CliInput::DefaultBasicRequest {
                save_as: Some(save_as),
                ..
            }
            | CliInput::BasicRequest {
                save_as: Some(save_as),
                ..
            } => Vec::from([factory_command_choices::save_as(
                save_as.to_string(),
                base_request.clone().unwrap(),
                None,
            )]),
            _ => vec![],
        };

        let save_current = match input {
            CliInput::Run {
                save: true,
                request_name,
                ..
            } => Vec::from([factory_command_choices::save(
                request_name.to_string(),
                base_request.clone().unwrap(),
            )]),
            _ => vec![],
        };

        save_as.extend(save_current);
        save_as
    };

    let main_commands: Vec<ViewCommandChoice> = match input {
        CliInput::Ls => vec![ViewCommandChoice::ShowRequests],
        CliInput::Inspect { request_name } => vec![ViewCommandChoice::InspectRequest {
            request_name: request_name.to_string(),
        }],
        CliInput::Remove { request_name } => vec![ViewCommandChoice::RemoveSavedRequest {
            request_name: request_name.to_string(),
        }],
        CliInput::Rename {
            request_name,
            new_name,
        } => vec![ViewCommandChoice::RenameSavedRequest {
            request_name: request_name.to_string(),
            new_name: new_name.to_string(),
        }],
        CliInput::DefaultBasicRequest { .. } => {
            vec![ViewCommandChoice::SubmitRequest {
                request: base_request.unwrap().to_request_data(),
            }]
        }
        CliInput::BasicRequest { .. } => {
            vec![ViewCommandChoice::SubmitRequest {
                request: base_request.unwrap().to_request_data(),
            }]
        }
        CliInput::Run { request_name, .. } => {
            vec![ViewCommandChoice::SubmitSavedRequest {
                request_name: request_name.to_string(),
                request_data: base_request.unwrap().clone(),
            }]
        }
        CliInput::Edit { request_name, .. } => {
            vec![ViewCommandChoice::SaveRequestWithBaseRequest {
                base_request_name: Some(request_name.to_string()),
                request_name: request_name.to_string(),
                request_data: base_request.unwrap(),
            }]
        }
    };

    Ok([pre_commands, main_commands]
        .into_iter()
        .flatten()
        .collect())
}

fn parser_request_items_to_data<'a>(
    base_request: OptionalRequestData,
    request_items: &'a [&'a str],
) -> OptionalRequestData {
    request_items
        .into_iter()
        .fold(base_request.clone(), |req_data, item| {
            [
                parsers_request_items::body_value,
                parsers_request_items::header_value,
            ]
            .into_iter()
            .find_map(|parser| parser(item, req_data.clone()))
            .unwrap_or(base_request.clone())
        })
}

mod parsers_request_items {
    use super::*;

    pub fn body_value(
        s: &str,
        mut base_request: OptionalRequestData,
    ) -> Option<OptionalRequestData> {
        let re = Regex::new(r"^(?<key>[ -~])=(?<value>[ -~])$").unwrap();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let original_body = base_request
            .body
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("{}");

        base_request.body = {
            let mut json =
                serde_json::from_str::<Map<String, Value>>(original_body).unwrap_or_default();
            json.insert(key.to_string(), Value::String(value.to_string()));
            serde_json::to_string(&json).unwrap_or_default().into()
        };

        Some(base_request)
    }

    pub fn header_value(
        s: &str,
        mut base_request: OptionalRequestData,
    ) -> Option<OptionalRequestData> {
        let re = Regex::new(r"^(?<key>[ -~]):(?<value>[ -~])$").unwrap();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        base_request
            .headers
            .get_or_insert(HashMap::new())
            .insert(key.to_string(), value.to_string());

        Some(base_request)
    }
}

mod factory_command_choices {
    use super::*;

    pub fn save_as(
        request_name: String,
        request_data: OptionalRequestData,
        base_request_name: Option<String>,
    ) -> ViewCommandChoice {
        ViewCommandChoice::SaveRequestWithBaseRequest {
            request_name,
            base_request_name,
            request_data,
        }
    }

    pub fn save(request_name: String, request_data: OptionalRequestData) -> ViewCommandChoice {
        ViewCommandChoice::SaveRequestWithBaseRequest {
            base_request_name: Some(request_name.clone()),
            request_name,
            request_data,
        }
    }
}
