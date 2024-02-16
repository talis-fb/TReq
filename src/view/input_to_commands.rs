use anyhow::Result;

use super::input::cli_input::CliInput;
use super::input::validators::basic_request_without_explicit_method::validate_basic_request_without_explicit_method;
use super::input::validators::body_values_with_raw::validate_body_values_with_raw;
use super::input::validators::url_alias::validate_alias_url_to_localhost;
use super::input_parsers::main_command_choices::parse_inputs_to_main_command_choices;
use super::input_parsers::request_data::parse_inputs_to_request_data;
use super::input_parsers::save_command_choices::parse_inputs_to_saving_command_choices;
use crate::view::commands::ViewCommandChoice;

pub fn map_input_to_commands(input: CliInput) -> Result<Vec<ViewCommandChoice>> {
    let input = Ok(input)
        .and_then(validate_basic_request_without_explicit_method)
        .and_then(validate_body_values_with_raw)
        .and_then(validate_alias_url_to_localhost)?;

    let base_request = parse_inputs_to_request_data(&input)?;
    let saving_commands_choice = parse_inputs_to_saving_command_choices(&input, &base_request)?;
    let main_command_choices = parse_inputs_to_main_command_choices(&input, &base_request)?;

    Ok([saving_commands_choice, main_command_choices]
        .into_iter()
        .flatten()
        .collect())
}
