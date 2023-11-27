use std::collections::HashMap;

use clap::{Parser, Subcommand};

use crate::app::services::request::entity::{RequestData, METHODS};
use crate::view::commands::AppCommand;


const USAGE: &str = r#"
Basic GET request
    treq [OPTIONS] [URL] 
Subcommands
    treq [OPTIONS] [COMMAND] [URL]

Examples
  Basic GET request (curl like), passing url as command
    $ treq example.com
    $ treq https://google.com

  Subcommands
    # Does same request of above first example
    $ treq get example.com

    # A POST request with specified body
    $ treq post example.com -b '{ "name": "John Doe" }'

    # PUT request, set the header 'Content-Type:application/json' and a empty json as body
    $ treq put example.com --json -b '{}'

    # POST request, with a custom header
    $ treq post example.com -b '{}' --header Authorization=None

    # POST request storing the body of response in 'output.json' file
    $ treq post example.com -b '{}' > output_body.json
"#;

#[derive(Parser)]
#[command(author, version, about, long_about = None, override_usage=USAGE)]
pub struct CliArgs {

    // ------------------------------
    // Subcommands
    // ------------------------------
    pub url_manual: Option<String>,

    /// Subcommand as the method to use in request
    #[command(subcommand)]
    pub command: Option<Commands>,

    // ------------------------------
    // Request args
    // ------------------------------

    /// Sets the body raw value of request
    #[arg(short, long, value_name = "BODY")]
    pub body: Option<String>,

    /// Sets a custom header to request, you must use 'key=value' format
    #[arg(long, value_name = "HEADER")]
    pub header: Vec<String>,

    /// Sets automatically the Content-Type:application/json in headers
    #[arg(long)]
    pub json: bool,

    // ------------------------------
    // Options
    // ------------------------------
    // #[arg(short, long)]
    // pub verbose: bool,
    //
    // #[arg(short, long)]
    // pub quiet: bool,
    //
    // #[arg(long)]
    // pub confirm: bool,
    //
    // #[arg(long)]
    // pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    /// Does a GET request
    GET {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },

    /// Does a POST request
    POST {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
        // /// Sets a custom header to request
        // #[arg(long, value_name = "BODY FROM STDIN")]
        // pub body_stdin: bool,

        // /// Sets a custom header to request
        // #[arg(long, value_name = "BODY FROM FILE")]
        // pub body_file: PathBuf,
    },

    /// Does a PUT request
    PUT {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },

    /// Does a PATCH request
    PATCH {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },

    /// Does a DELETE request
    DELETE {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },

    /// Does a OPTIONS request
    OPTIONS {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },

    /// Does a HEAD request
    HEAD {
        /// The url of Request
        url: String,

        /// Sets the body raw value of request
        #[arg(short, long, value_name = "BODY CONTENT")]
        body: Option<String>,

        /// Sets a custom header to request, you must use 'key=value' format
        #[arg(long, value_name = "KEY=VALUE")]
        header: Vec<String>,

        /// Sets automatically the Content-Type:application/json in headers
        #[arg(long)]
        json: bool,
    },
}

pub fn parse_cli_args_to_app_command(args: CliArgs) -> AppCommand {
    // Overwrite all possible config if a input has a url manual
    if let Some(url) = args.url_manual {
        return AppCommand::BasicRequest {
            req: build_request_data_with_subcommand_params(
                METHODS::GET,
                url,
                args.body,
                args.header,
                args.json,
            ),
        };
    }

    let commad = args.command.unwrap();

    let request_to_do = match commad {
        Commands::GET {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::GET, url, body, header, json),
        Commands::POST {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
        Commands::PATCH {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
        Commands::PUT {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
        Commands::DELETE {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
        Commands::HEAD {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
        Commands::OPTIONS {
            url,
            body,
            header,
            json,
        } => build_request_data_with_subcommand_params(METHODS::POST, url, body, header, json),
    };

    AppCommand::BasicRequest { req: request_to_do }
}

fn build_request_data_with_subcommand_params(
    method: METHODS,
    url: String,
    body: Option<String>,
    header: Vec<String>,
    json: bool,
) -> RequestData {
    let mut headers: HashMap<String, String> = header
        .iter()
        .filter_map(|s| s.split_once('='))
        .map(|(k, v)| (k.into(), v.into()))
        .collect();

    if json {
        headers.insert("Content-Type".into(), "application/json".into());
    }

    RequestData::default()
        .with_method(method)
        .with_url(url)
        .with_headers(headers)
        .with_body(body.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::{parse_cli_args_to_app_command, CliArgs, Commands};
    use crate::app::services::request::entity::RequestData;
    use crate::view::commands::AppCommand;

    #[test]
    fn with_only_url() {
        let input = CliArgs {
            url_manual: Some("url.com".into()),
            body: None,
            command: None,
            header: vec![],
            json: false,
        };

        let output = parse_cli_args_to_app_command(input);

        assert_eq!(
            output,
            AppCommand::BasicRequest {
                req: RequestData::default().with_url("url.com")
            }
        );
    }

    #[test]
    fn with_get_method() {
        let input = CliArgs {
            url_manual: None,
            body: None,
            header: vec![],
            command: Some(Commands::GET {
                url: "url.com".into(),
                body: Some("{}".into()),
                header: vec![],
                json: false,
            }),
            json: false,
        };

        let output = parse_cli_args_to_app_command(input);

        assert_eq!(
            output,
            AppCommand::BasicRequest {
                req: RequestData::default().with_url("url.com").with_body("{}")
            }
        );
    }
}
