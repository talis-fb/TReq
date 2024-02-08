#![allow(unused_variables)]
use std::io::{stderr, stdout};

use async_trait::async_trait;
use serde::Serialize;

use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::{OptionalRequestData, RequestData};
use crate::view::output::writer::CrosstermCliWriter;

pub mod inspect_request;
pub mod save_new_request;
pub mod save_request_with_base_request;
pub mod show_list_all_request;
pub mod submit_request;
pub mod submit_saved_request;

#[async_trait]
pub trait ViewCommand {
    async fn execute(self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()>;
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ViewCommandChoice {
    SubmitRequest {
        request: RequestData,
    },

    SubmitSavedRequest {
        request_name: String,
        request_data: OptionalRequestData,
    },

    SaveNewRequest {
        request_name: String,
        request_data: RequestData,
    },
    SaveRequestWithBaseRequest {
        request_name: String,
        base_request_name: Option<String>,
        request_data: OptionalRequestData,
    },

    RemoveSavedRequest {
        request_name: String,
    },

    RenameSavedRequest {
        request_name: String,
        new_name: String,
    },

    ShowRequests,
    InspectRequest {
        request_name: String,
    },
}

impl ViewCommandChoice {
    pub fn get_executor(self) -> Box<dyn ViewCommand> {
        use self::inspect_request::InspectRequestExecutor;
        use self::save_new_request::SaveNewRequestExecutor;
        use self::save_request_with_base_request::SaveRequestWithBaseRequestExecutor;
        use self::show_list_all_request::ShowListAllRequestExecutor;
        use self::submit_request::BasicRequestExecutor;
        use self::submit_saved_request::SubmitSavedRequestExecutor;

        let writer_stdout = CrosstermCliWriter::from(stdout());
        let writer_stderr = CrosstermCliWriter::from(stderr());

        match self {
            ViewCommandChoice::SubmitRequest { request } => BasicRequestExecutor {
                request,
                writer_stdout,
                writer_stderr,
            }
            .into(),
            ViewCommandChoice::SubmitSavedRequest {
                request_name,
                request_data,
            } => SubmitSavedRequestExecutor {
                request_name,
                request_data,
                writer_stdout,
                writer_stderr,
            }
            .into(),

            ViewCommandChoice::SaveNewRequest {
                request_name,
                request_data,
            } => SaveNewRequestExecutor {
                request_name,
                request_data,
                writer_stdout,
                writer_stderr,
            }
            .into(),
            ViewCommandChoice::SaveRequestWithBaseRequest {
                request_name,
                base_request_name,
                request_data,
            } => SaveRequestWithBaseRequestExecutor {
                request_name,
                base_request_name,
                request_data,
                writer_stdout,
                writer_stderr,
            }
            .into(),

            ViewCommandChoice::ShowRequests => ShowListAllRequestExecutor {
                writer: writer_stdout,
            }
            .into(),
            ViewCommandChoice::InspectRequest { request_name } => InspectRequestExecutor {
                request_name,
                writer: writer_stdout,
            }
            .into(),

            ViewCommandChoice::RenameSavedRequest {
                request_name,
                new_name,
            } => todo!(),
            ViewCommandChoice::RemoveSavedRequest { request_name } => todo!(),
        }
    }
}

impl<T> From<T> for Box<dyn ViewCommand>
where
    T: ViewCommand + 'static,
{
    fn from(code: T) -> Self {
        Box::new(code)
    }
}