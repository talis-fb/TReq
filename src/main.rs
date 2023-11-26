// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use clap::Parser;
use treq::app::provider::AppProvider;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::view::cli::clap_parser::{parse_cli_args_to_command, CliArgs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    if args.url_manual.is_none() && args.command.is_none() {
        println!("Type treq help");
        return Ok(());
    }

    // ----------------------------
    //  VIEW
    // ----------------------------
    let command = parse_cli_args_to_command(args);
    let mut command_executor = command.get_executor();

    // ----------------------------
    //  BACKEND
    // ----------------------------
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let provider = AppProvider::init(req, web).await;

    // ----------------------------
    //  Execute command received
    // ----------------------------
    command_executor.execute(Box::new(provider)).await.unwrap();

    Ok(())
}
