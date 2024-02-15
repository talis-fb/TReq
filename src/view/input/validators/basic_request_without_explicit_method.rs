use anyhow::Result;
use predicates::Predicate;

use crate::app::services::request::entities::methods::METHODS;
use crate::view::input::cli_input::{CliCommandChoice, CliInput};

pub fn basic_request_without_explicit_method(mut input: CliInput) -> Result<CliInput> {
    if let CliCommandChoice::DefaultBasicRequest { ref url } = input.choice {
        let url = url.clone();
        input
            .request_input
            .request_items
            .iter()
            .any(|v| predicates::str::contains('=').count(1).eval(v))
            .then(|| {
                input.choice = CliCommandChoice::BasicRequest {
                    method: METHODS::POST,
                    url,
                };
            });
    }

    Ok(input)
}
