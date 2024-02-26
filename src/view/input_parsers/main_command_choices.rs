use anyhow::Result;

use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::view::commands::ViewCommandChoice;
use crate::view::input::cli_input::{CliCommandChoice, CliInput};

pub fn parse_inputs_to_main_command_choices(
    input: &CliInput,
    base_request: &PartialRequestData,
) -> Result<Vec<ViewCommandChoice>> {
    let main_commands: Vec<ViewCommandChoice> = match &input.choice {
        CliCommandChoice::Ls => vec![ViewCommandChoice::ShowRequests],
        CliCommandChoice::Inspect { request_name } => vec![ViewCommandChoice::InspectRequest {
            request_name: request_name.to_string(),
        }],
        CliCommandChoice::Remove { request_name } => vec![ViewCommandChoice::RemoveSavedRequest {
            request_name: request_name.to_string(),
        }],
        CliCommandChoice::Rename {
            request_name,
            new_name,
            has_to_confirm,
        } => vec![ViewCommandChoice::RenameSavedRequest {
            request_name: request_name.to_string(),
            new_name: new_name.to_string(),
            has_to_confirm: *has_to_confirm,
        }],
        CliCommandChoice::Edit { request_name } => {
            vec![ViewCommandChoice::SaveRequestWithBaseRequest {
                base_request_name: Some(request_name.to_string()),
                request_name: request_name.to_string(),
                request_data: base_request.clone(),
            }]
        }
        CliCommandChoice::Run { request_name, save } => {
            let main_command = ViewCommandChoice::SubmitSavedRequest {
                request_name: request_name.to_string(),
                request_data: base_request.clone(),
                view_options: input.view_options.clone(),
            };

            if *save {
                Vec::from([
                    ViewCommandChoice::SaveRequestWithBaseRequest {
                        base_request_name: Some(request_name.clone()),
                        request_name: request_name.clone(),
                        request_data: base_request.clone(),
                    },
                    main_command,
                ])
            } else {
                vec![main_command]
            }
        }
        CliCommandChoice::DefaultBasicRequest { .. } => {
            vec![ViewCommandChoice::SubmitRequest {
                request: base_request.clone().to_request_data(),
                view_options: input.view_options.clone(),
            }]
        }
        CliCommandChoice::BasicRequest { .. } => {
            vec![ViewCommandChoice::SubmitRequest {
                request: base_request.clone().to_request_data(),
                view_options: input.view_options.clone(),
            }]
        }
    };
    Ok(main_commands)
}
