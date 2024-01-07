use clap::{Arg, ArgAction, Command};

pub fn root_command() -> Command {
    let mut app = Command::new("treq");
    app = add_main_input_values_request(app);

    app = app
        .subcommand(
            [Command::new("GET")]
                .into_iter()
                .map(add_main_input_values_request)
                .map(add_requests_flag)
                .map(add_utils_flag)
                .next()
                .unwrap(),
        )
        .subcommand(
            [Command::new("POST")]
                .into_iter()
                .map(add_main_input_values_request)
                .map(add_requests_flag)
                .map(add_utils_flag)
                .next()
                .unwrap(),
        );

    app
}

fn add_main_input_values_request(command: Command) -> Command {
    command.arg(
        Arg::new("inputs")
            .value_name("inputs")
            .required(true)
            .num_args(1..)
            .help("All entrys"),
    )
}

fn add_requests_flag(command: Command) -> Command {
    command.arg(
        Arg::new("save-as")
            .long("save-as")
            .value_name("SAVE_NAME")
            .help("Before submit request, save it as a request permanent"),
    )
}

fn add_saved_requests_flag(command: Command) -> Command {
    command.arg(
        Arg::new("save")
            .long("save")
            .action(ArgAction::SetTrue)
            .help("Before submit request, save changes"),
    )
}

fn add_utils_flag(command: Command) -> Command {
    command.arg(
        Arg::new("no-color")
            .long("no-color")
            .action(ArgAction::SetTrue)
            .help("Print output in terminal without colors (default behavior if piped)"),
    )
}

// For save commands
fn add_requests_method_arg(command: Command) -> Command {
    command.arg(Arg::new("method"))
}
fn add_requests_url_arg(command: Command) -> Command {
    command.arg(
        Arg::new("url_manual")
            .short('u')
            .long("url")
            .value_name("URL")
            .help("Sets a url manual"),
    )
}
