use std::collections::HashMap;

use treq::services::provider::{Provider, ServicesProvider};
use treq::services::request::entity::{RequestData, METHODS};
use treq::services::request::service::RequestService;

#[tokio::main]
async fn main() {
    // Services
    let req = RequestService::init();

    // Provider
    let mut provider = ServicesProvider::init(req).await;

    // Tests
    let mut my_req = RequestData::default();
    let id = provider.add_request(my_req.clone()).await;

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    my_req.url = "google.com".into();
    provider.edit_request(id.clone(), my_req.clone()).await;

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    my_req.method = METHODS::PUT;
    my_req.headers = HashMap::from([("type".into(), "json".into())]);
    provider.edit_request(id.clone(), my_req.clone()).await;
    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    provider.delete_request(id.clone()).await;

    println!("Req {:?}", provider.get_request(id.clone()).await);
}
