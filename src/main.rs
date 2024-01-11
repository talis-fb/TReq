// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::sync::Arc;

use tokio::sync::Mutex;
use treq::app::backend::AppBackend;
use treq::app::services::files::service::FileService;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::view::cli::commands::get_executor_of_cli_command;
use treq::view::cli::input::clap_definition::root_command;
use treq::view::cli::input::parser::parse_clap_input_to_commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = root_command().get_matches();

    let cli_commands = parse_clap_input_to_commands(args).unwrap();
    let commands_executors = cli_commands.into_iter().map(get_executor_of_cli_command);

    // ----------------------------
    //  BACKEND
    // ----------------------------
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init("", "", "");
    let backend = AppBackend::init(req, web, files);
    let provider = Arc::new(Mutex::new(backend));

    // ----------------------------
    //  Execute commands
    // ----------------------------
    for executor in commands_executors {
        executor(provider.clone()).await??;
    }

    Ok(())
}
