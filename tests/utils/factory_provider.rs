use treq::app::backend::AppBackend;
use treq::app::services::files::service::FileService;
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::repository_client::HttpClientRepository;
use treq::app::services::web_client::service::WebClient;

pub async fn create_default_provider() -> AppBackend {
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init("", "", "");
    AppBackend::init(req, web, files)
}

pub async fn create_provider_with_mock_web_client(
    web: impl HttpClientRepository + 'static,
) -> AppBackend {
    let req = RequestService::init();
    let web = WebClient::init(web);
    let files = FileService::init("", "", "");
    AppBackend::init(req, web, files)
}
