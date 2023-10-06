use treq::services::{request::RequestService, provider::{ServicesProvider, Provider}};


mod services;

#[tokio::main]
async fn main() {
    let req = RequestService::init();

    let mut provider = ServicesProvider::init(req);

    provider.add_request();
}
