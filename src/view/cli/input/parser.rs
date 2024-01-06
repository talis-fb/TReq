use std::collections::HashMap;

use clap::ArgMatches;

use crate::app::services::request::entity::{RequestData, METHODS};
use crate::view::cli::commands::CliCommand;
use crate::view::cli::validators;

pub fn parse(args: ArgMatches) -> Result<CliCommand, String> {
    if args.subcommand().is_none() {
        let request = mount_req_with_args(&args)?;
        return Ok(CliCommand::BasicRequest { request });
    }

    let request_from_subcommand = match args.subcommand().unwrap() {
        ("GET", args) => {
            let mut request = mount_req_with_args(&args)?;
            request.method = METHODS::GET;
            Some(request)
        }
        ("POST", args) => {
            let mut request = mount_req_with_args(&args)?;
            request.method = METHODS::POST;
            Some(request)
        }
        ("PUT", args) => {
            let mut request = mount_req_with_args(&args)?;
            request.method = METHODS::PUT;
            Some(request)
        }
        ("DELETE", args) => {
            let mut request = mount_req_with_args(&args)?;
            request.method = METHODS::DELETE;
            Some(request)
        }
        _ => None,
    };

    if let Some(request) = request_from_subcommand {
        return Ok(CliCommand::BasicRequest { request });
    }

    Err("No mapped subcommand".to_owned())
}

fn mount_req_with_args(args: &ArgMatches) -> Result<RequestData, String> {
    let inputs: Vec<&String> = args.get_many::<String>("inputs").unwrap().collect();
    Ok(mount_req(inputs)?)
}

fn mount_req(inputs: Vec<&String>) -> Result<RequestData, String> {
    let mut url: Option<String> = None;
    let mut headers = HashMap::<String, String>::new();

    inputs.into_iter().for_each(|input| {
        if url.is_none() && validators::is_url(&input) {
            url = Some(input.to_owned());
            return;
        }

        if validators::is_header_input(&input) {
            let (key, value) = input.split_once(':').unwrap();
            headers.insert(key.to_owned(), value.to_owned());
        }
    });

    let url = url.ok_or("No url provided")?;

    let request = RequestData::default().with_headers(headers).with_url(url);

    Ok(request)
}
