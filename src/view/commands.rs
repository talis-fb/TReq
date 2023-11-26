use std::io::stdout;

use async_trait::async_trait;

use crate::app::provider::Provider;
use crate::app::services::request::entity::RequestData;
use crate::view::cli::writer::CrosstermCliWriter;
use crate::view::commands::submit_request::BasicRequestExecutor;

pub mod submit_request;

#[async_trait]
pub trait AppCommandExecutor {
    async fn execute(&mut self, provider: Box<dyn Provider + Send>) -> anyhow::Result<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppCommand {
    /// A basic GET request
    BasicRequest { req: RequestData },
}

impl AppCommand {
    pub fn get_executor(&self) -> Box<dyn AppCommandExecutor> {
        let stdout = stdout();
        let writer = CrosstermCliWriter { stdout };

        use AppCommand as E;
        match self {
            E::BasicRequest { req } => Box::new(BasicRequestExecutor {
                req: req.clone(),
                writer,
            }),
        }
    }
}
