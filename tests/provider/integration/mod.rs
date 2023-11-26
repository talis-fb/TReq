use anyhow::Result;
use serde::{Deserialize, Serialize};
use treq::app::provider::{AppProvider, Provider};
use treq::app::services::request::entity::{RequestData, METHODS};

use crate::provider::factory_provider::create_default_provider;

const DEFAULT_HOST_MOCK_API: &str = "http://localhost:7777";
fn mock_server_url_with_route<'a>(route: impl Into<&'a str>) -> String {
    let mut url = std::env::var("HOST_MOCK_API").unwrap_or(DEFAULT_HOST_MOCK_API.into());
    url.push_str(route.into());
    url
}

#[tokio::test]
async fn test_single_request_get() -> Result<()> {
    let mut provider = create_default_provider().await;

    let mut basic_request = RequestData::default()
        .with_name("Basic")
        .with_url(mock_server_url_with_route("/ping/"));

    let id = provider.add_request(basic_request.clone()).await?;

    let response = provider.submit_request_blocking(id.clone()).await?;
    assert_eq!(200, response.status);

    basic_request = basic_request.with_url(mock_server_url_with_route("/status/403/"));

    provider
        .edit_request(id.clone(), basic_request.clone())
        .await?;

    let response = provider.submit_request_blocking(id.clone()).await?;
    assert_eq!(403, response.status);

    Ok(())
}

#[tokio::test]
async fn test_requests_body_get() -> Result<()> {
    let mut provider = create_default_provider().await;

    let basic_request1 = RequestData::default().with_url(mock_server_url_with_route("/hello/John"));

    let basic_request2 =
        RequestData::default().with_url(mock_server_url_with_route("/hello/Brian"));

    let basic_request3 =
        RequestData::default().with_url(mock_server_url_with_route("/hello/another-name"));

    make_and_validate_request(&mut provider, basic_request1, "John").await;
    make_and_validate_request(&mut provider, basic_request2, "Brian").await;
    make_and_validate_request(&mut provider, basic_request3, "another-name").await;

    async fn make_and_validate_request(
        provider: &mut AppProvider,
        req: RequestData,
        message_expected: &str,
    ) {
        // Model of response
        #![allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        struct ResponseBodyExpected {
            Hello: String,
        }
        let id_req = provider.add_request(req.clone()).await.unwrap();
        let response = provider.submit_request_blocking(id_req).await.unwrap();
        let response_body: ResponseBodyExpected = serde_json::from_str(&response.body).unwrap();

        assert_eq!(
            ResponseBodyExpected {
                Hello: message_expected.to_string(),
            },
            response_body,
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
        .with_url(mock_server_url_with_route("/echo/"))
        .with_method(METHODS::POST)
        .with_body(r#"{ "name": "John Doe", "foo": "bar" }"#);

    let id_req = provider.add_request(request.clone()).await.unwrap();
    let response = provider.submit_request_blocking(id_req).await.unwrap();
    let response_body: PayloadBody = serde_json::from_str(&response.body).unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(
        PayloadBody {
            name: "John Doe".into(),
            foo: "bar".into(),
        },
        response_body,
    );

    Ok(())
}
