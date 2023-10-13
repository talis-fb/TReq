use treq::services::provider::{Provider, ServicesProvider};
use treq::services::request::entity::RequestData;
use treq::services::request::service::RequestService;

#[tokio::main]
async fn main() {
    let req = RequestService::init();
    let mut provider = ServicesProvider::init(req).await;
    provider.add_request(RequestData::default()).await;
    println!("Aqui tudo morre");
}
