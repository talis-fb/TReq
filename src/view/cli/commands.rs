#![allow(unused_variables)]
use std::io::{stderr, stdout};

use async_trait::async_trait;
use serde::Serialize;

use self::inspect_request::InspectRequestExecutor;
use self::save_new_request::SaveNewRequestExecutor;
use self::save_request_with_base_request::SaveRequestWithBaseRequestExecutor;
use self::show_list_all_request::ShowListAllRequestExecutor;
use self::submit_request::BasicRequestExecutor;
use self::submit_saved_request::SubmitSavedRequestExecutor;
use crate::app::backend::Backend;
use crate::app::services::request::entities::requests::{OptionalRequestData, RequestData};
use crate::view::cli::output::writer::CrosstermCliWriter;

pub mod inspect_request;
pub mod save_new_request;
pub mod save_request_with_base_request;
pub mod show_list_all_request;
pub mod submit_request;
pub mod submit_saved_request;

#[async_trait]
pub trait CliCommand {
    async fn execute(self: Box<Self>, provider: &mut dyn Backend) -> anyhow::Result<()>;
}

impl<T: CliCommand + 'static> From<T> for Box<dyn CliCommand> {
    fn from(code: T) -> Self {
        Box::new(code)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum CliCommandChoice {
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

pub fn get_executor_of_cli_command(command: CliCommandChoice) -> Box<dyn CliCommand> {
    let writer_stdout = CrosstermCliWriter::from(stdout());
    let writer_stderr = CrosstermCliWriter::from(stderr());

    match command {
        CliCommandChoice::SubmitRequest { request } => BasicRequestExecutor {
            request,
            writer_stdout,
            writer_stderr,
        }
        .into(),
        CliCommandChoice::SubmitSavedRequest {
            request_name,
            request_data,
        } => SubmitSavedRequestExecutor {
            request_name,
            request_data,
            writer_stdout,
            writer_stderr,
        }
        .into(),

        CliCommandChoice::SaveNewRequest {
            request_name,
            request_data,
        } => SaveNewRequestExecutor {
            request_name,
            request_data,
            writer_stdout,
            writer_stderr,
        }
        .into(),
        CliCommandChoice::SaveRequestWithBaseRequest {
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

        CliCommandChoice::ShowRequests => ShowListAllRequestExecutor {
            writer: writer_stdout,
        }
        .into(),
        CliCommandChoice::InspectRequest { request_name } => InspectRequestExecutor {
            request_name,
            writer: writer_stdout,
        }
        .into(),

        CliCommandChoice::RenameSavedRequest {
            request_name,
            new_name,
        } => todo!(),
        CliCommandChoice::RemoveSavedRequest { request_name } => todo!(),
    }
}
