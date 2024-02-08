use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;

use crate::app::services::request::entities::methods::METHODS;

pub struct CliInputData<'a> {
    pub request_items: RequestItems<'a>,
    pub choice: CliInput<'a>,
    pub saving: SaveInputs<'a>,
}

pub enum CliInput<'a> {
    DefaultBasicRequest {
        url: &'a str,
    },
    BasicRequest {
        method: METHODS,
        url: &'a str,
    },
    Run {
        request_name: &'a str,
        save: bool,
    },
    Edit {
        request_name: &'a str,
    },
    Remove {
        request_name: &'a str,
    },
    Rename {
        request_name: &'a str,
        new_name: &'a str,
    },
    Inspect {
        request_name: &'a str,
    },
    Ls,
}

#[derive(Default)]
pub struct RequestItems<'a> {
    pub request_items: Vec<&'a str>,
    pub raw_body: Option<&'a str>,
    pub url_manual: Option<&'a str>,
    pub method_manual: Option<METHODS>,
}

#[derive(Default)]
pub struct SaveInputs<'a> {
    pub save_as: Option<&'a str>,
}

impl<'a> CliInputData<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Result<CliInputData> {
        if matches.subcommand().is_none() {
            let inputs = clap_args_utils::get_many(matches, "inputs")?;

            let url = inputs[0];
            let request_items = inputs[1..].to_vec();

            let raw_body = clap_args_utils::get_one(matches, "raw");
            let save_as = clap_args_utils::get_one(matches, "save_as");

            return Ok(CliInputData {
                choice: CliInput::DefaultBasicRequest { url },
                request_items: RequestItems {
                    request_items,
                    raw_body,
                    url_manual: None,
                    method_manual: None,
                },
                saving: SaveInputs { save_as },
            });
        }

        let subcommand = matches.subcommand().unwrap();

        match subcommand {
            ("edit", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;

                let request_name = inputs[0];
                let request_items = clap_args_utils::get_request_items(matches)?;
                let saving = clap_args_utils::get_save_inputs(matches);

                Ok(CliInputData {
                    request_items,
                    saving,
                    choice: CliInput::Edit { request_name },
                })
            }
            ("rename", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;
                let request_name = inputs[0];
                let new_name = inputs[1];

                Ok(CliInputData {
                    choice: CliInput::Rename {
                        request_name,
                        new_name,
                    },
                    request_items: RequestItems::default(),
                    saving: SaveInputs::default(),
                })
            }
            ("remove", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;
                let request_name = inputs[0];

                Ok(CliInputData {
                    choice: CliInput::Remove { request_name },
                    request_items: RequestItems::default(),
                    saving: SaveInputs::default(),
                })
            }
            ("ls", _) => Ok(CliInputData {
                choice: CliInput::Ls,
                request_items: RequestItems::default(),
                saving: SaveInputs::default(),
            }),
            ("inspect", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;
                let request_name = inputs[0];

                Ok(CliInputData {
                    choice: CliInput::Inspect { request_name },
                    request_items: RequestItems::default(),
                    saving: SaveInputs::default(),
                })
            }
            ("run", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;
                let request_name = inputs[0];
                let should_save_current_request = matches.get_one::<bool>("save").unwrap_or(&false);

                let request_items = clap_args_utils::get_request_items(matches)?;
                let saving = clap_args_utils::get_save_inputs(matches);

                Ok(CliInputData {
                    choice: CliInput::Run {
                        request_name,
                        save: *should_save_current_request,
                    },
                    request_items,
                    saving,
                })
            }
            ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs")?;
                let url = inputs[0];
                let method = METHODS::from_str(subcommand.0)?;

                let request_items = clap_args_utils::get_request_items(matches)?;
                let saving = clap_args_utils::get_save_inputs(matches);

                Ok(CliInputData {
                    choice: CliInput::BasicRequest { method, url },
                    request_items,
                    saving,
                })
            }
            _ => Err(Error::msg("No valid subcommand")),
        }
    }
}

mod clap_args_utils {
    use super::*;

    pub fn get_request_items<'a>(args: &'a ArgMatches) -> Result<RequestItems<'a>> {
        Ok(RequestItems {
            request_items: clap_args_utils::get_many(args, "inputs")?[1..].to_vec(),
            raw_body: clap_args_utils::get_one(args, "raw"),
            url_manual: clap_args_utils::get_one(args, "url_manual"),
            method_manual: clap_args_utils::get_one(args, "method_manual")
                .and_then(|m| METHODS::from_str(m).ok()),
        })
    }

    pub fn get_save_inputs<'a>(args: &'a ArgMatches) -> SaveInputs<'a> {
        SaveInputs {
            save_as: clap_args_utils::get_one(args, "save-as"),
        }
    }

    pub fn get_many<'a>(args: &'a ArgMatches, name: &str) -> Result<Vec<&'a str>> {
        Ok(args
            .try_get_many::<String>(name)?
            .ok_or(Error::msg("No input given"))?
            .map(|s| s.as_str())
            .collect::<Vec<_>>())
    }

    pub fn get_one<'a>(args: &'a ArgMatches, name: &str) -> Option<&'a str> {
        args.try_get_one::<String>(name)
            .ok()
            .flatten()
            .map(|s| s.as_str())
    }
}
