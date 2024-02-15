use anyhow::{Error, Result};
use predicates::Predicate;
use serde_json::{Map, Value};

use crate::view::input::cli_input::CliInput;

pub fn validate_body_values_with_raw(input: CliInput) -> Result<CliInput> {
    if let Some(raw_value) = &input.request_input.raw_body {
        let is_raw_input_a_valid_map =
            serde_json::from_str::<Map<String, Value>>(raw_value).is_ok();

        if !is_raw_input_a_valid_map {
            let has_some_body_insert_in_request_items = input
                .request_input
                .request_items
                .iter()
                .any(|v| predicates::str::contains('=').count(1).eval(v));

            if has_some_body_insert_in_request_items {
                return Err(Error::msg("raw body must be a valid JSON object"));
            }
        }
    }

    Ok(input)
}
