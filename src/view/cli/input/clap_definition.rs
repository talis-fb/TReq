use clap::{Arg, ArgAction, Command, ValueHint};

pub fn root_command() -> Command {
    Command::new("treq")
        .arg(
            Arg::new("url_default")
                .value_name("URL")
                .num_args(1..)
                .help("Saves the response to a file"),
        )
        .subcommand(
            Command::new("GET").arg(
                Arg::new("url")
                    .value_hint(ValueHint::FilePath)
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("POST").arg(Arg::new("url").required(true).index(1)))
        .subcommand(Command::new("PUT").arg(Arg::new("url").required(true).index(1)))
        .subcommand(Command::new("DELETE").arg(Arg::new("url").required(true).index(1)))
        .arg(
            Arg::new("json")
                .long("json")
                .help("Sets the request content type to JSON"),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .value_name("URL")
                .help("Overrides the URL specified in the subcommand"),
        )
        .arg(
            Arg::new("method")
                .long("method")
                .value_name("METHOD")
                .help("Overrides the HTTP method"),
        )
        .arg(
            Arg::new("save-as")
                .long("save-as")
                .action(ArgAction::SetTrue)
                .help("Saves the response to a file"),
        )
}
