use std::env::temp_dir;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::{oneshot, Mutex};
use treq::app::backend::{AppBackend, Backend};
use treq::app::services::files::service::FileService;
use treq::app::services::request::entities::{OptionalRequestData, RequestData, METHODS};
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::entities::Response;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::utils::uuid::UUID;
use treq::view::cli::command_executors;
use treq::view::cli::output::writer::CliWriterRepository;
use treq::view::style::StyledStr;

use crate::mocks::repositories::{create_mock_back_end, CliWriterUseLess};

#[tokio::test]
async fn should_submit_a_basic_request() {
    let mut backend = create_mock_back_end();
    let request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::POST)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    backend.set_expected_requests([request_to_do.clone()]);

    use command_executors::submit_request::basic_request_executor;

    let executor = basic_request_executor(request_to_do, CliWriterUseLess, CliWriterUseLess);
    executor(Arc::new(Mutex::new(backend)))
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn should_submit_a_request_after_saved() -> anyhow::Result<()> {
    let mut backend = create_mock_back_end();

    let first_request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::GET)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    let request_after_changes = OptionalRequestData {
        method: Some(METHODS::POST),
        body: Some(r#"{ "Hello": "World" }"#.into()),
        url: None,
        headers: None,
    };

    backend.set_expected_requests([
        first_request_to_do.clone(),
        request_after_changes.clone().to_request_data(),
    ]);

    use command_executors::save_request::save_request_executor;
    use command_executors::submit_request::basic_request_executor;
    use command_executors::submit_saved_request::submit_saved_request_executor;

    let backend = Arc::new(Mutex::new(backend));

    basic_request_executor(first_request_to_do, CliWriterUseLess, CliWriterUseLess)(
        backend.clone(),
    )
    .await??;

    save_request_executor(
        "my_request".into(),
        request_after_changes.clone(),
        false,
        CliWriterUseLess,
        CliWriterUseLess,
    )(backend.clone())
    .await??;

    submit_saved_request_executor(
        "my_request".into(),
        OptionalRequestData::default(),
        CliWriterUseLess,
        CliWriterUseLess,
    )(backend.clone())
    .await??;

    Ok(())
}
