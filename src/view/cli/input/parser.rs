use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;
use serde_json::Value;

use crate::app::services::request::entities::{OptionalRequestData, METHODS};
use crate::utils::validators;
use crate::view::cli::commands::CliCommandChoice;
use crate::view::cli::input::clap_definition::root_command;

pub fn parse_clap_input_to_commands(args: ArgMatches) -> Result<Vec<CliCommandChoice>> {
    if args.subcommand().is_none() {
        let (url, inputs_data) = get_inputs_from_clap_matches_splitted(&args).map_err(|err| {
            root_command().print_help().unwrap();
            err
        })?;

        if !validators::is_url(url) {
            return Err(Error::msg(format!("Invalid URL: {url}")));
        }

        let mut request_data = parse_list_of_data_to_request_data(inputs_data.to_vec())?;
        request_data.url = Some(url.to_string());
        request_data.method = request_data.method.or_else(|| {
            if request_data.body.is_some() {
                Some(METHODS::POST)
            } else {
                Some(METHODS::GET)
            }
        });

        let mut commands = Vec::new();

        commands.append(&mut parses_input_to_commands::save_as(
            &args,
            request_data.clone(),
            None,
        ));

        commands.push(CliCommandChoice::SubmitRequest {
            request: request_data.to_request_data(),
        });

        return Ok(commands);
    }

    let subcommand = args.subcommand().unwrap();

    match subcommand {
        ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH", matches) => {
            let (url, inputs_data) = get_inputs_from_clap_matches_splitted(matches)?;

            if !validators::is_url(url) {
                return Err(Error::msg(format!("Invalid URL: {url}")));
            }

            let mut request_data = parse_list_of_data_to_request_data(inputs_data.to_vec())?;
            request_data.url = Some(url.to_string());
            request_data.method = Some(METHODS::from_str(subcommand.0)?);

            if let Some(METHODS::GET) = request_data.method {
                request_data.body = None;
            }

            request_data = [parses_input_to_request_data::raw_body]
                .into_iter()
                .try_fold(request_data, |acc, f| f(matches, acc))?;

            let mut commands = Vec::new();
            commands.append(&mut parses_input_to_commands::save_as(
                matches,
                request_data.clone(),
                None,
            ));
            commands.push(CliCommandChoice::SubmitRequest {
                request: request_data.to_request_data(),
            });
            Ok(commands)
        }
        ("edit", matches) => {
            let (name_saved_request, inputs_data) = get_inputs_from_clap_matches_splitted(matches)?;

            let mut request_data = parse_list_of_data_to_request_data(inputs_data.to_vec())?;

            request_data = [
                parses_input_to_request_data::url_manual,
                parses_input_to_request_data::method_manual,
                parses_input_to_request_data::raw_body,
            ]
            .into_iter()
            .try_fold(request_data, |acc, f| f(matches, acc))?;

            let mut commands = Vec::new();
            commands.append(&mut parses_input_to_commands::save_as(
                matches,
                request_data.clone(),
                Some(name_saved_request.to_string()),
            ));
            commands.push(CliCommandChoice::SaveRequestWithBaseRequest {
                request_name: name_saved_request.to_string(),
                base_request_name: Some(name_saved_request.to_string()),
                request_data,
            });

            Ok(commands)
        }
        ("rename", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let old_name = inputs[0];
            let new_name = inputs[1];
            Ok(Vec::from([CliCommandChoice::RenameSavedRequest {
                request_name: old_name.to_string(),
                new_name: new_name.to_string(),
            }]))
        }
        ("remove", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let request_name = inputs[0].to_string();
            Ok(Vec::from([CliCommandChoice::RemoveSavedRequest {
                request_name,
            }]))
        }
        ("ls", _) => Ok(Vec::from([CliCommandChoice::ShowRequests])),
        ("inspect", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let request_name = inputs[0].to_string();
            Ok(Vec::from([CliCommandChoice::InspectRequest {
                request_name,
            }]))
        }
        ("run", matches) => {
            let (request_name, inputs_data) = get_inputs_from_clap_matches_splitted(matches)?;

            let mut request_data = parse_list_of_data_to_request_data(inputs_data)?;
            request_data = [
                parses_input_to_request_data::url_manual,
                parses_input_to_request_data::method_manual,
                parses_input_to_request_data::raw_body,
            ]
            .into_iter()
            .try_fold(request_data, |acc, f| f(matches, acc))?;

            let mut commands = Vec::new();

            commands.append(&mut parses_input_to_commands::save_as(
                matches,
                request_data.clone(),
                Some(request_name.to_string()),
            ));
            commands.append(&mut parses_input_to_commands::save(
                matches,
                request_data.clone(),
                request_name.clone(),
            ));

            commands.push(CliCommandChoice::SubmitSavedRequest {
                request_name: request_name.to_string(),
                request_data: request_data.clone(),
            });

            Ok(commands)
        }
        _ => Err(Error::msg("No valid subcommand")),
    }
}

mod parses_input_to_commands {
    use super::*;

    pub fn save_as(
        matches: &ArgMatches,
        request_data: OptionalRequestData,
        base_request_name: Option<String>,
    ) -> Vec<CliCommandChoice> {
        if let Some(request_name) = matches.get_one::<String>("save-as").cloned() {
            vec![CliCommandChoice::SaveRequestWithBaseRequest {
                request_name,
                base_request_name,
                request_data,
            }]
        } else {
            vec![]
        }
    }

    pub fn save(
        matches: &ArgMatches,
        request_data: OptionalRequestData,
        request_name: String,
    ) -> Vec<CliCommandChoice> {
        if let Some(&true) = matches.get_one::<bool>("save") {
            vec![CliCommandChoice::SaveRequestWithBaseRequest {
                base_request_name: Some(request_name.clone()),
                request_name,
                request_data,
            }]
        } else {
            vec![]
        }
    }
}

mod parses_input_to_request_data {
    use super::*;

    pub fn url_manual(
        matches: &ArgMatches,
        mut request_data: OptionalRequestData,
    ) -> anyhow::Result<OptionalRequestData> {
        let has_manual_url_flag = matches.get_one::<String>("url_manual");
        if let Some(url) = has_manual_url_flag {
            if !validators::is_url(url) {
                return Err(Error::msg(format!("Invalid URL: {url}")));
            }
            request_data.url = Some(url.clone());
        }
        Ok(request_data)
    }

    pub fn method_manual(
        matches: &ArgMatches,
        mut request_data: OptionalRequestData,
    ) -> anyhow::Result<OptionalRequestData> {
        let has_manual_method_flag = matches.get_one::<String>("method_manual");
        if let Some(method) = has_manual_method_flag {
            request_data.method = Some(METHODS::from_str(method)?);
        }
        Ok(request_data)
    }

    pub fn raw_body(
        matches: &ArgMatches,
        mut request_data: OptionalRequestData,
    ) -> anyhow::Result<OptionalRequestData> {
        let has_raw_body_input_flag = matches.get_one::<String>("raw");
        if let Some(raw_body) = has_raw_body_input_flag {
            if request_data.body.is_some() {
                return Err(Error::msg(
                    "You can't use --raw and --body at the same time".to_string(),
                ));
            }

            let _: Value =
                serde_json::from_str(raw_body).map_err(|err| Error::msg(format!("{err}")))?;

            request_data.body = Some(raw_body.to_string());
        }

        Ok(request_data)
    }
}

fn get_inputs_from_clap_matches(args: &ArgMatches) -> Result<Vec<&String>> {
    Ok(args
        .get_many::<String>("inputs")
        .ok_or(Error::msg("No inputs at command"))?
        .collect())
}

fn get_inputs_from_clap_matches_splitted(args: &ArgMatches) -> Result<(&String, Vec<&String>)> {
    let mut inputs = args
        .get_many::<String>("inputs")
        .ok_or(Error::msg("No inputs at command"))?
        .into_iter();

    let first_input: &String = inputs.next().unwrap();
    let rest: Vec<&String> = inputs.collect();
    Ok((first_input, rest))
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
