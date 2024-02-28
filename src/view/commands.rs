#![allow(unused_variables)]
use std::io::{empty, stderr, stdout};

use async_trait::async_trait;
use serde::Serialize;

use super::input::cli_input::ViewOptions;
use crate::app::backend::Backend;
use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::app::services::request::entities::requests::RequestData;
use crate::view::output::writer::CrosstermCliWriter;

pub mod inspect_request;
pub mod remove_request;
pub mod rename_request;
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
        view_options: ViewOptions,
    },

    SubmitSavedRequest {
        request_name: String,
        request_data: PartialRequestData,
        view_options: ViewOptions,
    },

    SaveNewRequest {
        request_name: String,
        request_data: RequestData,
        view_options: ViewOptions,
    },
    SaveRequestWithBaseRequest {
        request_name: String,
        base_request_name: Option<String>,
        request_data: PartialRequestData,
        view_options: ViewOptions,
    },

    RemoveSavedRequest {
        request_name: String,
        view_options: ViewOptions,
    },

    RenameSavedRequest {
        request_name: String,
        new_name: String,
        has_to_confirm: bool,
        view_options: ViewOptions,
    },

    ShowRequests,
    InspectRequest {
        request_name: String,
    },
}

impl ViewCommandChoice {
    pub fn get_executor(self) -> Box<dyn ViewCommand> {
        use self::inspect_request::InspectRequestExecutor;
        use self::remove_request::RemoveRequestExecutor;
        use self::rename_request::RenameRequestExecutor;
        use self::save_new_request::SaveNewRequestExecutor;
        use self::save_request_with_base_request::SaveRequestWithBaseRequestExecutor;
        use self::show_list_all_request::ShowListAllRequestExecutor;
        use self::submit_request::BasicRequestExecutor;
        use self::submit_saved_request::SubmitSavedRequestExecutor;

        match self {
            ViewCommandChoice::SubmitRequest {
                request,
                view_options,
            } => if view_options.print_body_only {
                BasicRequestExecutor {
                    request,
                    writer_metadata: CrosstermCliWriter::from(empty()),
                    writer_response: CrosstermCliWriter::from(stdout()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            } else if view_options.suppress_output {
                BasicRequestExecutor {
                    request,
                    writer_metadata: CrosstermCliWriter::from(empty()),
                    writer_response: CrosstermCliWriter::from(empty()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            } else {
                BasicRequestExecutor {
                    request,
                    writer_metadata: CrosstermCliWriter::from(stderr()),
                    writer_response: CrosstermCliWriter::from(stdout()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            }
            .into(),

            ViewCommandChoice::SubmitSavedRequest {
                request_name,
                request_data,
                view_options,
            } => if view_options.print_body_only {
                SubmitSavedRequestExecutor {
                    request_name,
                    input_request_data: request_data,
                    writer_metadata: CrosstermCliWriter::from(empty()),
                    writer_response: CrosstermCliWriter::from(stdout()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            } else if view_options.suppress_output {
                SubmitSavedRequestExecutor {
                    request_name,
                    input_request_data: request_data,
                    writer_metadata: CrosstermCliWriter::from(empty()),
                    writer_response: CrosstermCliWriter::from(empty()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            } else {
                SubmitSavedRequestExecutor {
                    request_name,
                    input_request_data: request_data,
                    writer_metadata: CrosstermCliWriter::from(stderr()),
                    writer_response: CrosstermCliWriter::from(stdout()),
                    writer_stderr: CrosstermCliWriter::from(stderr()),
                }
            }
            .into(),

            ViewCommandChoice::SaveNewRequest {
                request_name,
                request_data,
                view_options,
            } => if view_options.suppress_output {
                SaveNewRequestExecutor {
                    request_name,
                    request_data,
                    writer: CrosstermCliWriter::from(Box::new(empty())),
                }
            } else {
                SaveNewRequestExecutor {
                    request_name,
                    request_data,
                    writer: CrosstermCliWriter::from(Box::new(stderr())),
                }
            }
            .into(),

            ViewCommandChoice::SaveRequestWithBaseRequest {
                request_name,
                base_request_name,
                request_data,
                view_options,
            } => if view_options.suppress_output {
                SaveRequestWithBaseRequestExecutor {
                    request_name,
                    base_request_name,
                    input_request_data: request_data,
                    writer: CrosstermCliWriter::from(Box::new(empty())),
                }
            } else {
                SaveRequestWithBaseRequestExecutor {
                    request_name,
                    base_request_name,
                    input_request_data: request_data,
                    writer: CrosstermCliWriter::from(Box::new(stderr())),
                }
            }
            .into(),

            ViewCommandChoice::ShowRequests => ShowListAllRequestExecutor {
                writer: CrosstermCliWriter::from(Box::new(stdout())),
            }
            .into(),

            ViewCommandChoice::InspectRequest { request_name } => InspectRequestExecutor {
                request_name,
                writer: CrosstermCliWriter::from(Box::new(stdout())),
            }
            .into(),

            ViewCommandChoice::RemoveSavedRequest {
                request_name,
                view_options,
            } => if view_options.suppress_output {
                RemoveRequestExecutor {
                    request_name,
                    writer: CrosstermCliWriter::from(Box::new(empty())),
                }
            } else {
                RemoveRequestExecutor {
                    request_name,
                    writer: CrosstermCliWriter::from(Box::new(stdout())),
                }
            }
            .into(),

            ViewCommandChoice::RenameSavedRequest {
                request_name,
                new_name,
                has_to_confirm,
                view_options,
            } => if view_options.suppress_output {
                RenameRequestExecutor {
                    request_name,
                    new_name,
                    has_to_confirm,
                    writer: CrosstermCliWriter::from(Box::new(empty())),
                }
            } else {
                RenameRequestExecutor {
                    request_name,
                    new_name,
                    has_to_confirm,
                    writer: CrosstermCliWriter::from(Box::new(stdout())),
                }
            }
            .into(),
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
