use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;
use serde_json::Value;

use crate::app::services::request::entities::{OptionalRequestData, METHODS};
use crate::view::cli::commands::CliCommand;
use crate::view::cli::validators;

pub fn parse_clap_input_to_commands(args: ArgMatches) -> Result<Vec<CliCommand>> {
    if args.subcommand().is_none() {
        let inputs = get_inputs_from_clap_matches(&args)?;
        let (url, extra_inputs) = inputs
            .split_first()
            .ok_or(Error::msg("No inputs"))
            .and_then(|(url, rest)| match validators::is_url(url) {
                true => Ok((url, rest)),
                false => Err(Error::msg(format!("Invalid URL: {url}"))),
            })?;

        let mut optional_request = parse_list_of_data_to_request_data(extra_inputs.to_vec())?;
        optional_request.url = Some(url.to_string());

        optional_request.method = optional_request.method.or_else(|| {
            if optional_request.body.is_some() {
                Some(METHODS::POST)
            } else {
                Some(METHODS::GET)
            }
        });

        let mut commands = Vec::new();

        let has_save_as_flag = args.get_one::<String>("save-as");
        if let Some(request_name) = has_save_as_flag {
            commands.push(CliCommand::SaveRequest {
                request_data: optional_request.clone(),
                request_name: request_name.clone(),
                check_exists_before: false,
            })
        }

        let request = optional_request.to_request_data();
        commands.push(CliCommand::SubmitRequest { request });

        return Ok(commands);
    }

    let subcommand = args.subcommand().unwrap();

    match subcommand {
        ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let method = METHODS::from_str(subcommand.0)?;

            let (url, extra_inputs) = inputs
                .split_first()
                .ok_or(Error::msg("No inputs"))
                .and_then(|(url, rest)| match validators::is_url(url) {
                    true => Ok((url, rest)),
                    false => Err(Error::msg(format!("Invalid URL: {url}"))),
                })?;

            let mut optional_request = parse_list_of_data_to_request_data(extra_inputs.to_vec())?;
            optional_request.url = Some(url.to_string());
            optional_request.method = Some(method);

            optional_request.body = optional_request.body.and_then(|body| {
                if method == METHODS::GET {
                    None
                } else {
                    Some(body)
                }
            });

            let has_raw_body_input_flag = matches.get_one::<String>("raw");
            if let Some(raw_body) = has_raw_body_input_flag {
                if optional_request.body.is_some() {
                    return Err(Error::msg(
                        "You can't use --raw and --body at the same time".to_string(),
                    ));
                }

                let _: Value =
                    serde_json::from_str(raw_body).map_err(|err| Error::msg(format!("{err}")))?;

                optional_request.body = Some(raw_body.to_string());
            }

            let mut commands = Vec::new();

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequest {
                    request_data: optional_request.clone(),
                    request_name: request_name.clone(),
                    check_exists_before: false,
                })
            }

            let request = optional_request.to_request_data();

            commands.push(CliCommand::SubmitRequest { request });

            Ok(commands)
        }
        ("edit", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let (name_saved_request, extra_inputs) =
                inputs.split_first().ok_or(Error::msg("No inputs"))?;
            let mut optional_request_data =
                parse_list_of_data_to_request_data(extra_inputs.to_vec())?;

            let mut commands = Vec::new();

            let has_manual_url_flag = matches.get_one::<String>("url_manual");
            if let Some(url) = has_manual_url_flag {
                optional_request_data.url = Some(url.clone());
            }

            let has_manual_method_flag = matches.get_one::<String>("method_manual");
            if let Some(method) = has_manual_method_flag {
                optional_request_data.method = Some(METHODS::from_str(method)?);
            }

            commands.push(CliCommand::SaveRequest {
                request_data: optional_request_data.clone(),
                request_name: name_saved_request.to_string(),
                check_exists_before: true,
            });

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequest {
                    request_data: optional_request_data,
                    request_name: request_name.clone(),
                    check_exists_before: false,
                })
            }

            Ok(commands)
        }
        ("rename", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let old_name = inputs[0];
            let new_name = inputs[1];
            Ok(Vec::from([CliCommand::RenameSavedRequest {
                request_name: old_name.to_string(),
                new_name: new_name.to_string(),
            }]))
        }
        ("remove", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let request_name = inputs[0].to_string();
            Ok(Vec::from([CliCommand::RemoveSavedRequest { request_name }]))
        },
        ("ls", _) => {
            Ok(Vec::from([CliCommand::ShowRequests]))
        },
        ("run", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;

            let request_name = inputs[0].to_string();
            let extra_inputs = &inputs[1..];

            let mut commands = Vec::new();

            let mut optional_request_data =
                parse_list_of_data_to_request_data(extra_inputs.to_vec())?;

            let has_manual_url_flag = matches.get_one::<String>("url_manual");
            if let Some(url) = has_manual_url_flag {
                optional_request_data.url = Some(url.clone());
            }

            let has_manual_method_flag = matches.get_one::<String>("method_manual");
            if let Some(method) = has_manual_method_flag {
                optional_request_data.method = Some(METHODS::from_str(method)?);
            }

            let has_save_current_flag = matches.get_one::<bool>("save");
            if let Some(&true) = has_save_current_flag {
                commands.push(CliCommand::SaveRequest {
                    request_data: optional_request_data.clone(),
                    request_name: request_name.clone(),
                    check_exists_before: true,
                })
            }

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(new_request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequestWithBaseRequest {
                    request_name: new_request_name.clone(),
                    base_request_name: request_name.clone(),
                    request_data: optional_request_data.clone(),
                    check_exists_before: false,
                })
            }

            commands.push(CliCommand::SubmitSavedRequest {
                request_name,
                request_data: optional_request_data.clone(),
            });

            Ok(commands)
        }
        _ => Err(Error::msg("No valid subcommand")),
    }
}

fn get_inputs_from_clap_matches(args: &ArgMatches) -> Result<Vec<&String>> {
    Ok(args
        .get_many::<String>("inputs")
        .ok_or(Error::msg("No inputs at command"))?
        .collect())
}

fn parse_list_of_data_to_request_data<'a>(
    inputs: impl IntoIterator<Item = &'a String>,
) -> Result<OptionalRequestData> {
    let mut request = OptionalRequestData::default();
    let mut body_data_values = HashMap::new();

    inputs.into_iter().for_each(|input| {
        if validators::is_header_input(input) {
            let (key, value) = input.split_once(':').unwrap();
            request
                .headers
                .get_or_insert(HashMap::new())
                .insert(key.to_owned(), value.to_owned());
        } else if validators::is_body_data_input(input) {
            let (key, value) = input.split_once('=').unwrap();
            body_data_values.insert(key.to_owned(), value.to_owned());
        }
    });

    if !body_data_values.is_empty() {
        request.body = {
            let raw_json = serde_json::to_string(&body_data_values)
                .map_err(|err| Error::msg(format!("Error to parse data body to json: {}", err)))?;

            Some(raw_json)
        }
    }

    Ok(request)
}
