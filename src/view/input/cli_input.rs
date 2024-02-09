use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;

use crate::app::services::request::entities::methods::METHODS;

pub struct CliInput<'a> {
    pub choice: CliCommandChoice<'a>,
    pub request_input: RequestBuildingOptions<'a>,
    pub save_options: SavingOptions<'a>,
}

pub enum CliCommandChoice<'a> {
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

impl<'a> CliInput<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Result<CliInput> {
        if matches.subcommand().is_none() {
            let url = clap_args_utils::get_input(matches)?;
            let request_input = RequestBuildingOptions::from_clap_matches(matches)?;
            let save_options = SavingOptions::from_clap_matches(matches)?;

            return Ok(CliInput {
                choice: CliCommandChoice::DefaultBasicRequest { url },
                request_input,
                save_options,
            });
        }

        let (subcommand, matches) = matches.subcommand().unwrap();

        let request_input = RequestBuildingOptions::from_clap_matches(matches)?;
        let save_options = SavingOptions::from_clap_matches(matches)?;

        match subcommand {
            "edit" => {
                let request_name = clap_args_utils::get_input(matches)?;

                Ok(CliInput {
                    choice: CliCommandChoice::Edit { request_name },
                    request_input,
                    save_options,
                })
            }
            "rename" => {
                let inputs = clap_args_utils::get_many_inputs(matches)?;
                let request_name = inputs[0];
                let new_name = inputs[1];

                Ok(CliInput {
                    choice: CliCommandChoice::Rename {
                        request_name,
                        new_name,
                    },
                    request_input,
                    save_options,
                })
            }
            "remove" => {
                let request_name = clap_args_utils::get_input(matches)?;

                Ok(CliInput {
                    choice: CliCommandChoice::Remove { request_name },
                    request_input,
                    save_options,
                })
            }
            "ls" => Ok(CliInput {
                choice: CliCommandChoice::Ls,
                request_input,
                save_options,
            }),
            "inspect" => {
                let request_name = clap_args_utils::get_input(matches)?;

                Ok(CliInput {
                    choice: CliCommandChoice::Inspect { request_name },
                    request_input,
                    save_options,
                })
            }
            "run" => {
                let request_name = clap_args_utils::get_input(matches)?;
                let should_save_current_request = matches.get_one::<bool>("save").unwrap_or(&false);

                Ok(CliInput {
                    choice: CliCommandChoice::Run {
                        request_name,
                        save: *should_save_current_request,
                    },
                    request_input,
                    save_options,
                })
            }
            "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH" => {
                let url = clap_args_utils::get_input(matches)?;
                let method = METHODS::from_str(subcommand)?;

                Ok(CliInput {
                    choice: CliCommandChoice::BasicRequest { method, url },
                    request_input,
                    save_options,
                })
            }
            _ => Err(Error::msg("No valid subcommand")),
        }
    }
}

#[derive(Default)]
pub struct RequestBuildingOptions<'a> {
    pub request_items: Vec<&'a str>,
    pub raw_body: Option<&'a str>,
    pub url_manual: Option<&'a str>,
    pub method_manual: Option<METHODS>,
}
impl<'a> RequestBuildingOptions<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Result<RequestBuildingOptions> {
        Ok(RequestBuildingOptions {
            request_items: clap_args_utils::get_many(matches, "request-items").unwrap_or_default(),
            raw_body: clap_args_utils::get_one(matches, "raw"),
            url_manual: clap_args_utils::get_one(matches, "url-manual"),
            method_manual: clap_args_utils::get_one(matches, "method-manual")
                .and_then(|m| METHODS::from_str(m).ok()),
        })
    }
}

#[derive(Default)]
pub struct SavingOptions<'a> {
    pub save_as: Option<&'a str>,
}
impl<'a> SavingOptions<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Result<SavingOptions> {
        Ok(SavingOptions {
            save_as: clap_args_utils::get_one(matches, "save-as"),
        })
    }
}

mod clap_args_utils {
    use super::*;

    pub fn get_input<'a>(args: &'a ArgMatches) -> Result<&'a str> {
        clap_args_utils::get_one(args, "inputs").ok_or(Error::msg("No input given"))
    }

    pub fn get_many_inputs<'a>(args: &'a ArgMatches) -> Result<Vec<&'a str>> {
        clap_args_utils::get_many(args, "inputs").ok_or(Error::msg("No inputs given"))
    }

    pub fn get_one<'a>(args: &'a ArgMatches, name: &str) -> Option<&'a str> {
        args.try_get_one::<String>(name)
            .ok()
            .flatten()
            .map(|s| s.as_str())
    }

    pub fn get_many<'a>(args: &'a ArgMatches, name: &str) -> Option<Vec<&'a str>> {
        Some(
            args.try_get_many::<String>(name)
                .ok()??
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        )
    }
}
