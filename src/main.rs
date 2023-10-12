use treq::services::{request::RequestService, provider::{ServicesProvider, Provider}};


mod services;
mod utils;

#[tokio::main]
async fn main() {
    let req = RequestService::init();

    let mut provider = ServicesProvider::init(req).await;

    // provider.listen().await.unwrap();

    provider.add_request().await;

    println!("Aqui tudo morre");
}
