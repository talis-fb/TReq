use treq::app::services::request::entities::{OptionalRequestData, RequestData, METHODS};
use treq::view::cli::commands::{self, CliCommand};

use crate::mocks::repositories::{create_mock_back_end, CliWriterUseLess};

#[tokio::test]
async fn should_submit_a_basic_request() -> anyhow::Result<()> {
    let request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::POST)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    use commands::submit_request::BasicRequestExecutor;

    let executor: Box<dyn CliCommand> = BasicRequestExecutor {
        request: request_to_do.clone(),
        writer_stdout: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();

    let mut backend = create_mock_back_end().with_expected_requests([request_to_do]);

    executor.execute(&mut backend).await?;
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

    use commands::save_request::SaveRequestExecutor;
    use commands::submit_request::BasicRequestExecutor;
    use commands::submit_saved_request::SubmitSavedRequestExecutor;

    let mut backend = create_mock_back_end().with_expected_requests([
        first_request_to_do.clone(),
        request_after_changes.clone().to_request_data(),
    ]);

    let basic_request_executor: Box<dyn CliCommand> = BasicRequestExecutor {
        request: first_request_to_do,
        writer_stdout: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();
    basic_request_executor.execute(&mut backend).await?;

    let save_request_executor: Box<dyn CliCommand> = SaveRequestExecutor {
        request_name: "my_request".into(),
        request_data: request_after_changes.clone(),
        check_exists_before: false,
        writer_stdout: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();
    save_request_executor.execute(&mut backend).await?;

    let submit_save_request_executor: Box<dyn CliCommand> = SubmitSavedRequestExecutor {
        request_name: "my_request".into(),
        request_data: request_after_changes,
        writer_stdout: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();

    submit_save_request_executor.execute(&mut backend).await?;

    Ok(())
}
