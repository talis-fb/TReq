use anyhow::Result;

use crate::app::services::request::entities::methods::METHODS;
use crate::utils::regexes;
use crate::view::input::cli_input::{CliCommandChoice, CliInput};

pub fn validate_basic_request_without_explicit_method(mut input: CliInput) -> Result<CliInput> {
    if let CliCommandChoice::DefaultBasicRequest { ref url } = input.choice {
        let url = url.clone();
        input
            .request_input
            .request_items
            .iter()
            .any(|v| regexes::request_items::body_value().is_match(v))
            .then(|| {
                input.choice = CliCommandChoice::BasicRequest {
                    method: METHODS::POST,
                    url,
                };
            });
    }

    Ok(input)
}
