use treq::app::provider::Provider;
use treq::app::services::request::entity::RequestData;
use treq::app::services::web_client::entity::Response;
use treq::app::services::web_client::repository_client::MockHttpClientRepository;

use crate::provider::factory_provider::create_provider_with_mock_web_client;

#[tokio::test]
async fn test_basic_call_get() {
    fn expected_response() -> Response {
        let mut resp = Response::default();
        resp.status = 200;
        resp.body = "Ok".into();
        resp
    }

    let mut mock_client = MockHttpClientRepository::new();
    mock_client
        .expect_call_get()
        .times(1)
        .returning(move |_, _| tokio::task::spawn(async { Ok(expected_response()) }));

    let mut provider = create_provider_with_mock_web_client(mock_client).await;
    let id_req = provider.add_request(RequestData::default()).await.unwrap();

    let response_submit = provider.submit_request_blocking(id_req).await.unwrap();

    assert_eq!(response_submit, expected_response());
}
