// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use treq::app::provider::AppProvider;
use treq::app::services::files::service::FileService;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::view::cli::command_runners::{get_runner_of_command, CliCommandRunner};
use treq::view::cli::input::clap_definition::root_command;
use treq::view::cli::input::parser::parse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = root_command().get_matches();
    let command_to_exec = parse(args);
    if let Err(message) = command_to_exec {
        println!("ERRO: {message}");
        return Ok(());
    }

    let mut command_executor = get_runner_of_command(command_to_exec.unwrap());

    // ----------------------------
    //  BACKEND
    // ----------------------------
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init("", "", "");
    let provider = AppProvider::init(req, web, files).await;

    // ----------------------------
    //  Execute command received
    // ----------------------------
    command_executor.execute(provider).await?;

    Ok(())
}
