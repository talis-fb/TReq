#![allow(dead_code)]
use clap::{Arg, ArgAction, Command};

pub fn root_command() -> Command {
    let mut app = Command::new("treq");

    for method in ["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"] {
        app = app.subcommand(
            Some(Command::new(method))
                .map(add_main_input_values_request)
                .map(add_global_utils_flag)
                .map(add_raw_flag)
                .map(add_save_as_flag)
                .unwrap(),
        )
    }

    app = {
        app.subcommand(
            Some(Command::new("run"))
                .map(add_main_input_values_request)
                .map(add_global_utils_flag)
                .map(add_raw_flag)
                .map(add_save_as_flag)
                .map(add_save_changes_to_current_request_flag)
                .map(add_manual_url_flag)
                .map(add_manual_method_flag)
                .unwrap(),
        )
        .subcommand(
            Some(Command::new("edit"))
                .map(add_main_input_values_request)
                .map(add_global_utils_flag)
                .map(add_raw_flag)
                .map(add_save_as_flag)
                .map(add_manual_url_flag)
                .map(add_manual_method_flag)
                .unwrap(),
        )
        .subcommand(
            Command::new("remove").arg(
                Arg::new("inputs")
                    .value_name("inputs")
                    .required(true)
                    .num_args(1)
                    .help("All entrys"),
            ),
        )
        .subcommand(
            Command::new("rename").arg(
                Arg::new("inputs")
                    .value_name("inputs")
                    .required(true)
                    .num_args(2)
                    .help("All entrys"),
            ),
        )
    };

    app = app.subcommand(Command::new("collections"));

    // Running without a subcommand
    app = Some(app)
        .map(add_main_input_values_request)
        .map(add_global_utils_flag)
        .map(add_raw_flag)
        .map(add_save_as_flag)
        .unwrap();

    app
}

fn add_main_input_values_request(command: Command) -> Command {
    let is_required = !command.has_subcommands();

    command.arg(
        Arg::new("inputs")
            .value_name("inputs")
            .required(is_required)
            .num_args(1..)
            .help("All entrys"),
    )
}

fn add_raw_flag(command: Command) -> Command {
    command.arg(
        Arg::new("raw")
            .long("raw")
            .value_name("RAW_BODY_VALUE")
            .help("Raw body value"),
    )
}

fn add_save_as_flag(command: Command) -> Command {
    command.arg(
        Arg::new("save-as")
            .long("save-as")
            .value_name("SAVE_NAME")
            .help("Before submit request, save it as a request permanent"),
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
        Arg::new("method_manual")
            .long("method")
            .value_name("METHOD_MANUAL")
            .help("Set the HTTP Method when is not possible by subcommand"),
    )
}
fn add_manual_url_flag(command: Command) -> Command {
    command.arg(
        Arg::new("url_manual")
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

        assert_eq!(root_matches.get_one::<String>("inputs"), None);

        let (name_subcommand, matches_subcommand) = root_matches.subcommand().unwrap();
        assert_eq!(name_subcommand, "GET");
        assert!(matches_subcommand.args_present());

        let inputs: Vec<&String> = matches_subcommand
            .get_many::<String>("inputs")
            .unwrap()
            .collect();
        assert_eq!(inputs, vec!["https://httpbin.org/get"]);
    }

    // Test POST request passing 'save-as' parameter
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

        assert_eq!(root_matches.get_one::<String>("inputs"), None);

        let (name_subcommand, matches_subcommand) = root_matches.subcommand().unwrap();
        assert_eq!(name_subcommand, "POST");
        assert!(matches_subcommand.args_present());
    }
}
