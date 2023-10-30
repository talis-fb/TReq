use treq::app::provider::AppProvider;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::repository_client::HttpClientRepository;
use treq::app::services::web_client::service::WebClient;

pub async fn create_default_provider() -> AppProvider {
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    AppProvider::init(req, web).await
}

pub async fn create_provider_with_mock_web_client(
    web: impl HttpClientRepository + 'static,
) -> AppProvider {
    let req = RequestService::init();
    let web = WebClient::init(web);
    AppProvider::init(req, web).await
}
