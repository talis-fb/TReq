use std::sync::Arc;

use tokio::sync::Mutex;

use crate::app::backend::Backend;

pub mod remove_saved_request;
pub mod rename_saved_request;
pub mod save_request;
pub mod save_request_with_base_request;
pub mod show_list_all_request;
pub mod submit_request;
pub mod submit_saved_request;

pub type CommandExecutor =
    Box<dyn FnOnce(Arc<Mutex<dyn Backend>>) -> tokio::task::JoinHandle<anyhow::Result<()>>>;
