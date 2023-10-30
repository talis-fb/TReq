use anyhow::Result;
use serde::{Deserialize, Serialize};
use treq::app::provider::{AppProvider, Provider};
use treq::app::services::request::entity::{RequestData, METHODS};

use crate::provider::factory_provider::create_default_provider;

#[tokio::test]
async fn test_single_request_get() -> Result<()> {
    let mut provider = create_default_provider().await;

    let mut basic_request = RequestData::default()
        .with_name("Basic")
        .with_url("https://www.mockbin.com/status/200");

    let id = provider.add_request(basic_request.clone()).await?;

    let response = provider.submit_request(id.clone()).await?;
    assert_eq!(response.status, 200);

    basic_request = basic_request.with_url("https://www.mockbin.com/status/403");

    provider
        .edit_request(id.clone(), basic_request.clone())
        .await?;

    let response = provider.submit_request(id.clone()).await?;
    assert_eq!(response.status, 403);

    Ok(())
}

#[tokio::test]
async fn test_requests_body_get() -> Result<()> {
    // Model of response
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct ResponseBodyExpected {
        code: String,
        message: String,
    }

    let mut provider = create_default_provider().await;

    let basic_request1 = RequestData::default().with_url("https://www.mockbin.com/status/200/OK");

    let basic_request2 =
        RequestData::default().with_url("https://www.mockbin.com/status/200/some_message");

    let basic_request3 =
        RequestData::default().with_url("https://www.mockbin.com/status/403/another-message");

    let basic_request4 =
        RequestData::default().with_url("https://www.mockbin.com/status/500/internal-error");

    validate_request(&mut provider, basic_request1, 200, "OK").await;
    validate_request(&mut provider, basic_request2, 200, "some_message").await;
    validate_request(&mut provider, basic_request3, 403, "another-message").await;
    validate_request(&mut provider, basic_request4, 500, "internal-error").await;

    async fn validate_request(
        provider: &mut AppProvider,
        req: RequestData,
        code_expected: i32,
        message_expected: &str,
    ) {
        let id_req = provider.add_request(req.clone()).await.unwrap();
        let response = provider.submit_request(id_req).await.unwrap();
        let response_body: ResponseBodyExpected = serde_json::from_str(&response.body).unwrap();

        assert_eq!(response.status, code_expected);
        assert_eq!(
            response_body,
            ResponseBodyExpected {
                code: code_expected.to_string(),
                message: message_expected.into(),
            }
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_request_post() -> Result<()> {
    // Model of response
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct PayloadBody {
        name: String,
        foo: String,
    }

    let mut provider = create_default_provider().await;

    let request = RequestData::default()
        .with_url("https://www.mockbin.com/echo")
        .with_method(METHODS::POST)
        .with_body(r#"{ "name": "John Doe", "foo": "bar" }"#);

    let id_req = provider.add_request(request.clone()).await.unwrap();
    let response = provider.submit_request(id_req).await.unwrap();
    let response_body: PayloadBody = serde_json::from_str(&response.body).unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(
        response_body,
        PayloadBody {
            name: "John Doe".into(),
            foo: "bar".into(),
        }
    );

    Ok(())
}
