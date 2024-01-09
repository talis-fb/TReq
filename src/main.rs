// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use treq::app::provider::AppProvider;
use treq::app::services::files::service::FileService;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::view::cli::command_executors::{get_runner_of_command, CliCommandExecutor};
use treq::view::cli::input::clap_definition::root_command;
use treq::view::cli::input::parser::parse_clap_input_to_commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = root_command().get_matches();

    let commands_to_exec = parse_clap_input_to_commands(args);
    let commands_executors = match commands_to_exec {
        Ok(commands) => commands
            .into_iter()
            .map(|d| get_runner_of_command(d))
            .map(Box::new)
            .collect::<Vec<Box<_>>>(),
        Err(message) => {
            println!("ERRO: {message}");
            return Ok(());
        }
    };

    // ----------------------------
    //  BACKEND
    // ----------------------------
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init("", "", "");
    let mut provider = AppProvider::init(req, web, files).await;

    // ----------------------------
    //  Execute command received
    // ----------------------------
    for mut command in commands_executors {
        command.execute(&mut provider).await?;
    }

    Ok(())
}
