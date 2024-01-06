use async_trait::async_trait;

use crate::app::provider::Provider;

pub mod submit_request;

#[async_trait]
pub trait CliCommandRunner {
    async fn execute(&mut self, provider: impl Provider + Send) -> anyhow::Result<()>;
}
