use treq::app::services::request::entities::methods::METHODS;
use treq::app::services::request::entities::partial_entities::PartialRequestData;
use treq::app::services::request::entities::requests::{BodyPayload, RequestData};
use treq::app::services::request::entities::url::Url;
use treq::view::commands::{self, ViewCommand};

use crate::mocks::repositories::{create_mock_back_end, CliWriterUseLess};

#[tokio::test]
async fn should_submit_a_basic_request() -> anyhow::Result<()> {
    let request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::POST)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    use commands::submit_request::BasicRequestExecutor;

    let executor: Box<dyn ViewCommand> = BasicRequestExecutor {
        request: request_to_do.clone(),
        writer_metadata: CliWriterUseLess,
        writer_response: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();

    let mut backend = create_mock_back_end().with_expected_requests([request_to_do]);

    executor.execute(&mut backend).await?;
    Ok(())
}

#[tokio::test]
async fn should_submit_a_request_after_saved() -> anyhow::Result<()> {
    use commands::save_new_request::SaveNewRequestExecutor;
    use commands::save_request_with_base_request::SaveRequestWithBaseRequestExecutor;
    use commands::submit_request::BasicRequestExecutor;
    use commands::submit_saved_request::SubmitSavedRequestExecutor;

    let first_request_to_do = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::GET)
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    let input_second_request = PartialRequestData {
        url: Some(Url::from_str("https://google.com")),
        method: Some(METHODS::POST),
        body: Some(BodyPayload::Json(serde_json::json!({ "Hello": "World" }))),
        headers: None,
    };

    // Merge of first and input of second request
    let expected_second_request = RequestData::default()
        .with_url("https://google.com")
        .with_method(METHODS::POST)
        .with_body_payload(BodyPayload::Json(serde_json::json!({ "Hello": "World" })))
        .with_headers([("User-Agent".into(), "treq-test".into())]);

    let mut backend = create_mock_back_end()
        .with_expected_requests([first_request_to_do.clone(), expected_second_request.clone()]);

    let basic_request_executor: Box<dyn ViewCommand> = BasicRequestExecutor {
        request: first_request_to_do.clone(),
        writer_metadata: CliWriterUseLess,
        writer_response: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();
    basic_request_executor.execute(&mut backend).await?;

    let save_first_request_executor: Box<dyn ViewCommand> = SaveNewRequestExecutor {
        request_name: "some_request".into(),
        request_data: first_request_to_do,
        writer: CliWriterUseLess,
    }
    .into();
    save_first_request_executor.execute(&mut backend).await?;

    let save_request_executor: Box<dyn ViewCommand> = SaveRequestWithBaseRequestExecutor {
        base_request_name: Some("some_request".to_string()),
        request_name: "some_request".to_string(),
        input_request_data: input_second_request.clone(),
        writer: CliWriterUseLess,
    }
    .into();
    save_request_executor.execute(&mut backend).await?;

    let submit_save_request_executor: Box<dyn ViewCommand> = SubmitSavedRequestExecutor {
        request_name: "some_request".into(),
        input_request_data: PartialRequestData::default(),
        writer_metadata: CliWriterUseLess,
        writer_response: CliWriterUseLess,
        writer_stderr: CliWriterUseLess,
    }
    .into();

    submit_save_request_executor.execute(&mut backend).await?;

    Ok(())
}
