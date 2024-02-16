#![allow(dead_code)]
use clap::{command, Arg, ArgAction, Command};

pub fn root_command() -> Command {
    let mut app = command!();

    for method in ["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"] {
        app = app.subcommand(
            Some(
                Command::new(method)
                    .about(format!("Does a {method} request"))
                    .override_usage(format!("treq {method} <URL> [OPTIONS + REQUEST_ITENS... ]"))
                    .arg(Arg::new("inputs").value_name("URL").required(true)),
            )
            .map(add_request_items_args)
            .map(add_raw_flag)
            .map(add_save_as_flag)
            .unwrap(),
        )
    }

    app = {
        app.subcommand(
            Some(
                Command::new("run")
                    .override_usage(format!(
                        "treq run <REQUEST_NAME> [OPTIONS + REQUEST_ITENS... ]"
                    ))
                    .about("Submit saved request")
                    .arg(Arg::new("inputs").value_name("REQUEST_NAME").required(true)),
            )
            .map(add_request_items_args)
            .map(add_raw_flag)
            .map(add_save_as_flag)
            .map(add_save_changes_to_current_request_flag)
            .map(add_manual_url_flag)
            .map(add_manual_method_flag)
            .unwrap(),
        )
        .subcommand(
            Some(
                Command::new("edit")
                    .override_usage(format!(
                        "treq run <REQUEST_NAME> [OPTIONS + REQUEST_ITENS... ]"
                    ))
                    .about("Edit saved request data, it does not submit")
                    .arg(Arg::new("inputs").value_name("REQUEST_NAME").required(true)),
            )
            .map(add_request_items_args)
            .map(add_raw_flag)
            .map(add_save_as_flag)
            .map(add_manual_url_flag)
            .map(add_manual_method_flag)
            .unwrap(),
        )
        .subcommand(
            Command::new("remove")
                .override_usage(format!("treq run <REQUEST_NAME> [OPTIONS]"))
                .about("Remove request")
                .arg(
                    Arg::new("inputs")
                        .value_name("inputs")
                        .required(true)
                        .num_args(1)
                        .help("Saved request to remove"),
                ),
        )
        .subcommand(
            Command::new("rename")
                .override_usage(format!(
                    "treq run <OLD_REQUEST_NAME> <NEW_REQUEST_NAME> [OPTIONS]"
                ))
                .about("Rename request")
                .arg(
                    Arg::new("inputs")
                        .value_name("inputs")
                        .required(true)
                        .num_args(2)
                        .help("All entrys"),
                ),
        )
        .subcommand(Command::new("ls").about("List all saved requests"))
        .subcommand(
            Command::new("inspect")
                .about("Show request details and datas")
                .arg(
                    Arg::new("inputs")
                        .value_name("REQUEST_NAME")
                        .required(true)
                        .num_args(1)
                        .help("Request name to inspect"),
                ),
        )
    };

    // Running without a subcommand
    app = Some(app.arg(Arg::new("inputs").value_name("URL")))
        .map(add_request_items_args)
        .map(add_raw_flag)
        .map(add_save_as_flag)
        .unwrap();

    app = app.override_usage(
        r#"
- Basic GET/POST request
  $ treq [URL] [OPTIONS + REQUEST_ITENS]

- Request with specific method
  $ treq [HTTP_METHOD] [URL] [OPTIONS + REQUEST_ITENS]

- Another commands
  $ treq [SUBCOMMAND] [INPUT] [OPTIONS + REQUEST_ITENS]
"#,
    );

    app = app.after_help(
        r#"
----------------------------------------
For more information, see 'treq --help'
----------------------------------------
>>>> Feel free to submit any issue or pull requests on our GitHub repository.
>>>> https://github.com/talis-fb/TReq
"#,
    );

    app = app.after_long_help(
        r#"
Examples
  Basic GET request
    $ treq example.com
    $ treq https://google.com

    # With explicit method
    $ treq GET example.com

  Requests with additional data
    # POST request with custom Content-Type header
    $ treq POST example.com Content-Type:application/json

    # Same POST request passing a JSON object { "language": "Rust", "food": "pizza" }
    $ treq POST example.com language=Rust food=pizza

    # Using multiples datas together
    $ treq POST example.com Content-Type:application/json language=Rust
    $ treq POST example.com Authorization:None name="John Doe" language=Rust

  Saving requests
    # After requesting you can save it to do the same request later
    $ treq POST example.com name="John Doe" --save-as main-endpoint
    $ treq run main-endpoint

    # You can also edit the fields and insert new datas in each submit
    $ treq run main-endpoint name="Jane" another-field="value"
    $ treq run main-endpoint Authorization:None 

    # Or save it as a new request
    $ treq run main-endpoint job="dev" --save-as endpoint-with-job

    # You can also save the changes in same request after submit
    $ treq run main-endpoint name="Peter" --save

    # Or just edit request data without submit
    $ treq edit main-endpoint name="Michael" job="dev"

    # For more complex data, you can use JSON object directly with `--raw`
    $ treq run example.com --raw '{ names: ["John", "Doe"] }' 'Content-Type:application/json'

----------------------------------------
>>>> Feel free to submit any issue or pull requests on our GitHub repository.
>>>> https://github.com/talis-fb/TReq
"#
        .trim(),
    );

    app
}

fn add_request_items_args(command: Command) -> Command {
    command.arg(
        Arg::new("request-items")
            .value_name("REQUEST_ITENS")
            .num_args(1..)
            .help(
                r#"
Optional key-value pairs to be included in the request.
Like Header values, in data to be serialized in JSON at payload.
    Use `--help` for more details"#
                    .trim(),
            )
            .long_help(
                r#"
Optional key-value pairs to be included in the request. The separator used determines the type of:
    HTTP header: ':'
      Content-Type:application/json
      User-Agent:bacon/1.0
      Accept:application/json
      Accept-Language:en-US

    Body data fields be serialized into a JSON object: '='
      name=John 
      language=Rust
      country=Brazil
      description='The best CLI HTTP client'"#
                    .trim(),
            ),
    )
}

fn add_raw_flag(command: Command) -> Command {
    command.arg(
        Arg::new("raw")
            .long("raw")
            .value_name("RAW_PAYLOD")
            .help("Raw payload value to be used (This exclude REQUEST_ITENS)"),
    )
}

fn add_save_as_flag(command: Command) -> Command {
    command.arg(
        Arg::new("save-as")
            .long("save-as")
            .value_name("SAVE_NAME")
            .help("Save builded request as <SAVE_NAME>, a named request"),
    )
}

fn add_save_changes_to_current_request_flag(command: Command) -> Command {
    command.arg(
        Arg::new("save")
            .long("save")
            .action(ArgAction::SetTrue)
            .help("Before submit request, save changes"),
    )
}

fn add_global_utils_flag(command: Command) -> Command {
    command.arg(
        Arg::new("no-color")
            .long("no-color")
            .action(ArgAction::SetTrue)
            .help("Print output in terminal without colors (default behavior if piped)"),
    )
}

fn add_manual_method_flag(command: Command) -> Command {
    command.arg(
        Arg::new("method-manual")
            .long("method")
            .value_name("METHOD_MANUAL")
            .value_parser(["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"])
            .ignore_case(true)
            .help("Set the HTTP Method when is not possible by subcommand"),
    )
}
fn add_manual_url_flag(command: Command) -> Command {
    command.arg(
        Arg::new("url-manual")
            .short('u')
            .long("url")
            .value_name("URL")
            .help("Sets a url manual"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_get_request() {
        let root_matches = root_command()
            .try_get_matches_from(vec!["treq", "GET", "https://httpbin.org/get"])
            .unwrap();

        assert_eq!(None, root_matches.get_one::<String>("inputs"));
        assert_eq!(None, root_matches.get_one::<String>("request-items"));

        let (name_subcommand, matches_subcommand) = root_matches.subcommand().unwrap();
        assert_eq!("GET", name_subcommand);
        assert!(matches_subcommand.args_present());

        assert_eq!(
            vec!["https://httpbin.org/get"],
            matches_subcommand
                .get_many::<String>("inputs")
                .unwrap()
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_basic_get_request_with_request_items() {
        let root_matches = root_command()
            .try_get_matches_from(vec![
                "treq",
                "GET",
                "https://httpbin.org/get",
                "Hello=World",
                "Content:json",
                "job=dev",
            ])
            .unwrap();

        let (_, subcommand) = root_matches.subcommand().unwrap();

        assert_eq!(
            vec!["Hello=World", "Content:json", "job=dev"],
            subcommand
                .get_many::<String>("request-items")
                .unwrap()
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_basic_post_request() {
        let root_matches = root_command()
            .try_get_matches_from(vec![
                "treq",
                "POST",
                "https://httpbin.org/post",
                "--save-as",
                "test.json",
            ])
            .unwrap();

        assert_eq!(None, root_matches.get_one::<String>("inputs"));

        let (name_subcommand, matches_subcommand) = root_matches.subcommand().unwrap();
        assert_eq!("POST", name_subcommand);
        assert!(matches_subcommand.args_present());
    }

    #[test]
    fn test_basic_defatult_request() {
        let root_matches = root_command()
            .try_get_matches_from(vec![
                "treq",
                "https://httpbin.org/get",
                "--save-as",
                "test.json",
            ])
            .unwrap();

        assert_eq!(
            Some(&"https://httpbin.org/get".to_string()),
            root_matches.get_one::<String>("inputs"),
        );
        assert_eq!(None, root_matches.get_one::<String>("request-items"));
        assert_eq!(None, root_matches.subcommand());
    }

    #[test]
    fn test_basic_default_request_with_request_items() {
        let root_matches = root_command()
            .try_get_matches_from(vec![
                "treq",
                "https://httpbin.org/post",
                "Hello=World",
                "Content:json",
                "job=dev",
            ])
            .unwrap();

        assert_eq!(
            Some(&"https://httpbin.org/post".to_string()),
            root_matches.get_one::<String>("inputs"),
        );
        assert_eq!(
            vec!["Hello=World", "Content:json", "job=dev"],
            root_matches
                .get_many::<String>("request-items")
                .unwrap()
                .collect::<Vec<_>>(),
        );
        assert_eq!(root_matches.subcommand(), None);
    }
}
