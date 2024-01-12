use treq::app::backend::Backend;
use treq::app::services::request::entities::{RequestData, METHODS};

use crate::utils::factory_provider::create_default_provider;

#[tokio::test]
async fn test_create_single_request() {
    let mut provider = create_default_provider().await;

    let new_request = RequestData::default().with_url("duck.com");

    let id = provider.add_request(new_request.clone()).await.unwrap();

    let request_saved = provider.get_request(id).await.unwrap();

    assert!(request_saved.is_some());
    let request_saved = request_saved.unwrap();

    assert_eq!(new_request, (*request_saved).clone());
}

#[tokio::test]
async fn test_edit_single_request() {
    let mut provider = create_default_provider().await;

    // Save first
    let first_req = RequestData::default();
    let id = provider.add_request(first_req.clone()).await.unwrap();

    // Save second
    let second_req = first_req.with_url("google.com");

    provider
        .edit_request(id.clone(), second_req.clone())
        .await
        .unwrap();

    // Compare
    let new_request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*new_request_saved, second_req.clone());

    // Third
    let third_req = second_req
        .with_headers([("Content-type".into(), "json".into())])
        .with_method(METHODS::PUT);

    provider
        .edit_request(id.clone(), third_req.clone())
        .await
        .unwrap();

    // Compare
    let new_request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*new_request_saved, third_req);
}

#[tokio::test]
async fn test_delete_single_request() {
    let mut provider = create_default_provider().await;

    let new_request = RequestData::default().with_url("duck.com");
    let id = provider.add_request(new_request.clone()).await.unwrap();

    let request_saved = provider.get_request(id.clone()).await.unwrap();
    assert!(request_saved.is_some());

    provider.delete_request(id.clone()).await.unwrap();

    let request_saved = provider.get_request(id.clone()).await.unwrap();
    assert_eq!(request_saved, None);
}

#[tokio::test]
async fn test_rollback_single_request() {
    let mut provider = create_default_provider().await;

    let first_request = RequestData::default().with_url("google.com");

    let second_request = RequestData::default()
        .with_url("duck.com")
        .with_headers([("private".into(), "higher".into())]);

    let third_request = RequestData::default().with_url("bing.com").with_headers([]);

    let id = provider.add_request(first_request.clone()).await.unwrap();

    provider
        .edit_request(id.clone(), second_request.clone())
        .await
        .unwrap();
    provider
        .edit_request(id.clone(), third_request.clone())
        .await
        .unwrap();

    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, third_request.clone());

    provider.undo_request(id.clone()).await.unwrap();
    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, second_request.clone());

    provider.undo_request(id.clone()).await.unwrap();
    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, first_request.clone());

    // Remain the same if its the first
    provider.undo_request(id.clone()).await.unwrap();
    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, first_request.clone());

    provider.redo_request(id.clone()).await.unwrap();
    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, second_request.clone());

    provider.redo_request(id.clone()).await.unwrap();
    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, third_request.clone());

    provider.redo_request(id.clone()).await.unwrap();
    provider.redo_request(id.clone()).await.unwrap();
    provider.redo_request(id.clone()).await.unwrap();

    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, third_request.clone());

    provider.undo_request(id.clone()).await.unwrap();

    let request_saved = provider.get_request(id.clone()).await.unwrap().unwrap();
    assert_eq!(*request_saved, second_request.clone());
}
