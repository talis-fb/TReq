use anyhow::Result;

use crate::app::services::request::entities::partial_entities::PartialRequestData;
use crate::view::commands::ViewCommandChoice;
use crate::view::input::cli_input::{CliCommandChoice, CliInput};

pub fn parse_inputs_to_saving_command_choices(
    input: &CliInput,
    base_request: &PartialRequestData,
) -> Result<Vec<ViewCommandChoice>> {
    let save_commands: Vec<ViewCommandChoice> = input
        .save_options
        .save_as
        .clone()
        .map(|request_name| {
            let base_request_name = match input.choice {
                CliCommandChoice::Run {
                    ref request_name, ..
                }
                | CliCommandChoice::Edit {
                    ref request_name, ..
                } => Some(request_name.to_string()),
                _ => None,
            };

            Vec::from([ViewCommandChoice::SaveRequestWithBaseRequest {
                request_name,
                base_request_name,
                request_data: base_request.clone(),
            }])
        })
        .unwrap_or_default();

    Ok(save_commands)
}
