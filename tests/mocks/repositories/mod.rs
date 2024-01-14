use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::oneshot;
use treq::app::backend::{AppBackend, Backend};
use treq::app::services::files::service::FileService;
use treq::app::services::request::entities::{RequestData, METHODS};
use treq::app::services::request::service::RequestService;
use treq::app::services::web_client::entities::Response;
use treq::app::services::web_client::repository_client::reqwest::ReqwestClientRepository;
use treq::app::services::web_client::service::WebClient;
use treq::utils::uuid::UUID;
use treq::view::cli::command_executors::submit_request::basic_request_executor;
use treq::view::cli::output::writer::CliWriterRepository;
use treq::view::style::StyledStr;

pub fn create_mock_back_end() -> MockAppBackend {
    let temp_dir = std::env::temp_dir();

    let config_dir = temp_dir.join("config");
    let data_dir = temp_dir.join("data");
    let tempfiles_dir = temp_dir.join("tmp");

    [
        config_dir.as_path(),
        data_dir.as_path(),
        tempfiles_dir.as_path(),
    ]
    .iter()
    .filter(|dir| !dir.exists())
    .try_for_each(std::fs::create_dir_all)
    .unwrap();

    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init(config_dir, data_dir, tempfiles_dir);
    let backend = AppBackend::init(req, web, files);
    MockAppBackend::new(backend)
}

pub struct MockAppBackend {
    app_backend: AppBackend,
    expected_requests: Vec<RequestData>,
}

impl MockAppBackend {
    pub fn new(app_backend: AppBackend) -> Self {
        Self {
            app_backend,
            expected_requests: vec![],
        }
    }

    pub fn set_expected_requests(
        &mut self,
        expected_requests: impl IntoIterator<Item = RequestData>,
    ) {
        self.expected_requests = expected_requests.into_iter().collect();
    }
}

#[async_trait]
impl Backend for MockAppBackend {
    async fn submit_request_blocking(&mut self, id: UUID) -> Result<Response> {
        panic!("Not implemented");
    }

    async fn submit_request_async(
        &mut self,
        id: UUID,
    ) -> Result<oneshot::Receiver<Result<Response, String>>> {
        let request = self.app_backend.get_request(id).await?.unwrap();
        let expected_request = self.expected_requests.remove(0);
        assert_eq!(request, expected_request.into());

        let (tx, rx) = oneshot::channel();
        tx.send(Ok(Response::default())).unwrap();
        Ok(rx)
    }

    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        self.app_backend.add_request(request).await
    }

    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        self.app_backend.edit_request(id, request).await
    }

    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        self.app_backend.get_request(id).await
    }

    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.delete_request(id).await
    }

    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.undo_request(id).await
    }

    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.redo_request(id).await
    }

    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()> {
        self.app_backend
            .save_request_datas_as(name, request_data)
            .await
    }

    async fn get_request_saved(&mut self, name: String) -> Result<RequestData> {
        self.app_backend.get_request_saved(name).await
    }
}

pub struct CliWriterUseLess;

impl CliWriterRepository for CliWriterUseLess {
    fn clear_current_line(&mut self) {}

    fn print_lines<T: Display>(&mut self, _lines: impl IntoIterator<Item = T>) {}

    fn print_animation_single_line<T: Display, Sprites: IntoIterator<Item = T> + Sized + Clone>(
        &mut self,
        _sprites: Sprites,
        _interval: Duration,
        _finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone,
    {
    }

    fn print_centered_text_with_border(&mut self, _text: &str, _border_char: char) {}

    fn print_lines_styled<'a, StyledValues>(
        &mut self,
        _lines: impl IntoIterator<Item = StyledValues>,
    ) where
        StyledValues: IntoIterator<Item = StyledStr<'a>>,
    {
    }
}