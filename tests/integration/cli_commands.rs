use std::sync::Arc;

use tokio::sync::Mutex;
use treq::app::services::request::entities::{OptionalRequestData, RequestData, METHODS};
use treq::view::cli::command_executors;

use crate::mocks::repositories::{create_mock_back_end, CliWriterUseLess};

#[tokio::test]
async fn should_submit_a_basic_request() -> anyhow::Result<()> {
    let request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::POST)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    use command_executors::submit_request::basic_request_executor;

    let executor =
        basic_request_executor(request_to_do.clone(), CliWriterUseLess, CliWriterUseLess);

    let backend = Arc::new(Mutex::new(
        create_mock_back_end().with_expected_requests([request_to_do]),
    ));

    executor(backend).await??;
    Ok(())
}

#[tokio::test]
async fn should_submit_a_request_after_saved() -> anyhow::Result<()> {
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

    use command_executors::save_request::save_request_executor;
    use command_executors::submit_request::basic_request_executor;
    use command_executors::submit_saved_request::submit_saved_request_executor;

    let backend = Arc::new(Mutex::new(create_mock_back_end().with_expected_requests([
        first_request_to_do.clone(),
        request_after_changes.clone().to_request_data(),
    ])));

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
