use anyhow::Error;
use directories::ProjectDirs;
use treq::app::backend::AppBackend;
use treq::app::services::files::service::FileService;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::utils::errors::print_pretty_error;
use treq::view::input::cli_definition::root_command;
use treq::view::input::cli_input::CliInputData;
use treq::view::input_to_commands::map_input_to_commands;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    runner().await.map_err(print_pretty_error)
}

async fn runner() -> anyhow::Result<()> {
    let proj_dirs = ProjectDirs::from("com", APP_AUTHOR, APP_NAME).ok_or(Error::msg(
        "No possible to create or access directories of data and configuration",
    ))?;

    let config_dir = proj_dirs.config_dir();
    let data_dir = proj_dirs.data_dir();
    let tempfiles_dir = std::env::temp_dir();

    [config_dir, data_dir, tempfiles_dir.as_path()]
        .iter()
        .filter(|dir| !dir.exists())
        .try_for_each(std::fs::create_dir_all)?;

    // ----------------------------
    // Cli Input
    // ----------------------------
    let args = root_command().get_matches();
    let cli_inputs = CliInputData::from_clap_matches(&args)?;
    let cli_commands = map_input_to_commands(cli_inputs)?;
    let commands_executors = cli_commands.into_iter().map(|choice| choice.get_executor());

    // ----------------------------
    //  BACKEND
    // ----------------------------
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init(config_dir, data_dir, tempfiles_dir);
    let mut backend = AppBackend::init(req, web, files);

    // ----------------------------
    //  Execute commands
    // ----------------------------
    for executor in commands_executors {
        executor.execute(&mut backend).await?;
    }

    Ok(())
}
