use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use regex::Regex;
use serde_json::{Map, Value};

use super::input::cli_input::CliInputData;
use crate::app::services::request::entities::methods::METHODS;
use crate::app::services::request::entities::requests::{OptionalRequestData, Url};
use crate::view::commands::ViewCommandChoice;
use crate::view::input::cli_input::{CliInput, RequestItems};

pub fn map_input_to_commands(input: CliInputData) -> Result<Vec<ViewCommandChoice>> {
    // -------------------------------------
    // The builded requests by inputs
    //   It's necessary it be build in begginging
    //   because it can be used in "save-as" command down below
    //   Here is flags optionals in request_items
    // -------------------------------------
    let base_request: OptionalRequestData = {
        let RequestItems {
            raw_body,
            request_items,
            url_manual,
            method_manual,
        } = input.request_items;

        let mut req = OptionalRequestData::default();
        req.body = raw_body.map(String::from);
        req.method = method_manual;
        req.url = url_manual.and_then(|value| Url::from_str(value).ok());
        req = parser_request_items_to_data(req, &request_items);
        req
    };

    // The main params of input command to set in request.
    let base_request = match input.choice {
        CliInput::BasicRequest { method, url } => base_request.with_method(method).with_url(url),
        CliInput::DefaultBasicRequest { url } => {
            let method = if base_request.body.is_some() {
                METHODS::POST
            } else {
                METHODS::GET
            };
            base_request.with_method(method).with_url(url)
        }
        _ => base_request,
        
    };


    // -----------------------------------------------------
    // Commands to run before the main commands wished
    //   Theses commands are defined by optional flags
    //   like '--save-as' or '--save'
    // -----------------------------------------------------
    let save_commands: Vec<ViewCommandChoice> = {
        if let Some(request_name) = input.saving.save_as {
            let base_request_name = match input.choice {
                CliInput::Run { request_name, .. } | CliInput::Edit { request_name, .. } => {
                    Some(request_name.to_string())
                }
                _ => None,
            };

            Vec::from([factory_command_choices::save_as(
                request_name.to_string(),
                base_request.clone(),
                base_request_name,
            )])
        } else {
            vec![]
        }
    };

    // The main commands, run as the last on order
    let main_commands: Vec<ViewCommandChoice> = match input.choice {
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
                request: base_request.to_request_data(),
            }]
        }
        CliInput::BasicRequest { .. } => {
            vec![ViewCommandChoice::SubmitRequest {
                request: base_request.to_request_data(),
            }]
        }
        CliInput::Run { request_name, save } => {
            let main_command = ViewCommandChoice::SubmitSavedRequest {
                request_name: request_name.to_string(),
                request_data: base_request.clone(),
            };

            if save {
                Vec::from([
                    factory_command_choices::save(request_name.to_string(), base_request.clone()),
                    main_command,
                ])
            } else {
                vec![main_command]
            }
        }
        CliInput::Edit { request_name } => {
            vec![ViewCommandChoice::SaveRequestWithBaseRequest {
                base_request_name: Some(request_name.to_string()),
                request_name: request_name.to_string(),
                request_data: base_request,
            }]
        }
    };

    Ok([save_commands, main_commands]
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
        .fold(base_request, |req_data, item| {
            print!("AQI: {:?}", req_data);
            print!("AQI: {:?}", item);

            [
                parsers_request_items::body_value,
                parsers_request_items::header_value,
            ]
            .into_iter()
            .find_map(|parser| parser(item, &req_data))
            .unwrap_or(req_data)
        })
}

mod parsers_request_items {
    use super::*;

    pub fn body_value(s: &str, base_request: &OptionalRequestData) -> Option<OptionalRequestData> {
        let re = Regex::new(r"^(?<key>[ -~]+)=(?<value>[ -~]+)$").unwrap();
        let matcher = re.captures(s)?;

        let key = matcher.name("key")?.as_str();
        let value = matcher.name("value")?.as_str();

        let original_body = base_request
            .body
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("{}");

        let mut request = base_request.clone();

        request.body = {
            let mut json =
                serde_json::from_str::<Map<String, Value>>(original_body).unwrap_or_default();
            json.insert(key.to_string(), Value::String(value.to_string()));
            serde_json::to_string(&json).unwrap_or_default().into()
        };

        Some(request)
    }

    pub fn header_value(
        s: &str,
        base_request: &OptionalRequestData,
    ) -> Option<OptionalRequestData> {
        let re = Regex::new(r"^(?<key>[ -~]+):(?<value>[ -~]+)$").unwrap();
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
