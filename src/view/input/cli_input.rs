use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;

use crate::app::services::request::entities::methods::METHODS;

pub enum CliInput<'a> {
    DefaultBasicRequest {
        url: &'a str,
        request_items: Vec<&'a str>,
        raw_body: Option<&'a str>,
        save_as: Option<&'a str>,
    },
    BasicRequest {
        method: METHODS,
        url: &'a str,
        request_items: Vec<&'a str>,
        raw_body: Option<&'a str>,
        save_as: Option<&'a str>,
    },
    Run {
        request_name: &'a str,
        request_items: Vec<&'a str>,
        raw_body: Option<&'a str>,
        url_manual: Option<&'a str>,
        method_manual: Option<METHODS>,
        save: bool,
        save_as: Option<&'a str>,
    },
    Edit {
        request_name: &'a str,
        request_items: Vec<&'a str>,
        raw_body: Option<&'a str>,
        url_manual: Option<&'a str>,
        method_manual: Option<METHODS>,
        save_as: Option<&'a str>,
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

impl<'a> CliInput<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Result<CliInput> {
        if matches.subcommand().is_none() {
            let inputs = clap_args_utils::get_many(matches, "inputs");

            let url = inputs[0];
            let request_items = inputs[1..].to_vec();

            let raw_body = clap_args_utils::get_one(matches, "raw");
            let save_as = clap_args_utils::get_one(matches, "save_as");

            return Ok(CliInput::DefaultBasicRequest {
                url,
                request_items,
                raw_body,
                save_as,
            });
        }

        let subcommand = matches.subcommand().unwrap();

        match subcommand {
            ("edit", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");
                let request_name = inputs[0];
                let request_items = inputs[1..].to_vec();

                Ok(CliInput::Edit {
                    request_name,
                    request_items,
                    raw_body: clap_args_utils::get_one(matches, "raw"),
                    url_manual: clap_args_utils::get_one(matches, "url"),
                    save_as: clap_args_utils::get_one(matches, "save_as"),
                    method_manual: clap_args_utils::get_one(matches, "method")
                        .map(|m| METHODS::from_str(m))
                        .transpose()?,
                })
            }
            ("rename", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");
                let request_name = inputs[0];
                let new_name = inputs[1];

                Ok(CliInput::Rename {
                    request_name,
                    new_name,
                })
            }
            ("remove", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");
                let request_name = inputs[0];

                Ok(CliInput::Remove { request_name })
            }
            ("ls", _) => Ok(CliInput::Ls),
            ("inspect", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");
                Ok(CliInput::Inspect {
                    request_name: inputs[0],
                })
            }
            ("run", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");

                Ok(CliInput::Run {
                    request_name: inputs[0],
                    request_items: inputs[1..].to_vec(),
                    raw_body: clap_args_utils::get_one(matches, "raw"),
                    url_manual: clap_args_utils::get_one(matches, "url"),
                    method_manual: clap_args_utils::get_one(matches, "method")
                        .map(|m| METHODS::from_str(m))
                        .transpose()?,
                    save: clap_args_utils::get_one(matches, "save").is_some(),
                    save_as: clap_args_utils::get_one(matches, "save_as"),
                })
            }
            ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH", matches) => {
                let inputs = clap_args_utils::get_many(matches, "inputs");
                let url = inputs[0];
                let request_items = inputs[1..].to_vec();
                let method = METHODS::from_str(subcommand.0)?;

                Ok(CliInput::BasicRequest {
                    method,
                    url,
                    request_items,
                    raw_body: clap_args_utils::get_one(matches, "raw"),
                    save_as: clap_args_utils::get_one(matches, "save_as"),
                })
            }
            _ => Err(Error::msg("No valid subcommand")),
        }
    }
}

mod clap_args_utils {
    use clap::ArgMatches;

    pub fn get_many<'a>(args: &'a ArgMatches, name: &str) -> Vec<&'a str> {
        let matches: Option<_> = args.try_get_many::<String>(name).ok().flatten();

        match matches {
            Some(values) => values.map(|s| s.as_str()).collect(),
            None => Vec::new(),
        }
    }

    pub fn get_one<'a>(args: &'a ArgMatches, name: &str) -> Option<&'a str> {
        args.try_get_one::<String>(name)
            .ok()
            .flatten()
            .map(|s| s.as_str())
    }
}
