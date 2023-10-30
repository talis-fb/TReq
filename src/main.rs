#![allow(dead_code)]

use std::collections::HashMap;

use treq::app::provider::{AppProvider, Provider};
use treq::app::services::request::entity::{RequestData, METHODS};
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;

#[tokio::main]
async fn main() {
    // Services
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);

    // Provider
    let mut provider = AppProvider::init(req, web).await;

    // Tests
    let mut my_req = RequestData::default();
    let id = provider.add_request(my_req.clone()).await.unwrap();

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    my_req.url = "google.com".into();
    provider
        .edit_request(id.clone(), my_req.clone())
        .await
        .unwrap();

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    my_req.method = METHODS::PUT;
    my_req.headers = HashMap::from([("type".into(), "json".into())]);
    provider
        .edit_request(id.clone(), my_req.clone())
        .await
        .unwrap();

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    provider.undo_request(id.clone()).await.unwrap();

    println!("Req {:?}", provider.get_request(id.clone()).await.unwrap());

    provider.delete_request(id.clone()).await.unwrap();

    println!("Req {:?}", provider.get_request(id.clone()).await);
}
