use anyhow::Result;
use regex::Regex;

use crate::view::input::cli_input::{CliCommandChoice, CliInput, RequestBuildingOptions};

pub fn validate_alias_url_to_localhost(mut input: CliInput) -> Result<CliInput> {
    let url_to_validate = match &mut input {
        CliInput {
            choice: CliCommandChoice::BasicRequest { url, .. },
            ..
        }
        | CliInput {
            choice: CliCommandChoice::DefaultBasicRequest { url, .. },
            ..
        }
        | CliInput {
            request_input:
                RequestBuildingOptions {
                    url_manual: Some(url),
                    ..
                },
            ..
        } => url,
        _ => return Ok(input),
    };

    if let Some(url) = alias_url_to_localhost(url_to_validate) {
        *url_to_validate = url;
    }

    Ok(input)
}

fn alias_url_to_localhost(url: &str) -> Option<String> {
    let re = Regex::new(r"^:(?<port>[0-9]{1,6})?(\/(?<tail>[ -~]*))?$").unwrap();
    let matcher = re.captures(&url)?;

    let port = matcher
        .name("port")
        .map(|m| m.as_str())
        .map(|port| format!(":{port}"))
        .unwrap_or_default();

    let tail = matcher.name("tail").map(|m| m.as_str()).unwrap_or_default();

    Some(format!("localhost{port}/{tail}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_url_to_localhost() {
        let cases = [
            ("localhost/", ":/"),
            ("localhost:80/", ":80"),
            ("localhost:80/", ":80/"),
            ("localhost:80/route", ":80/route"),
            ("localhost:80/route/subroute/", ":80/route/subroute/"),
            ("localhost/route/subroute/", ":/route/subroute/"),
        ];

        for (expected, input) in cases {
            let output = alias_url_to_localhost(input);
            debug_assert!(output.is_some(), "input: {input}");
            assert_eq!(expected, output.unwrap().as_str());
        }
    }

    #[test]
    fn test_alias_url_to_localhost_with_not_match_urls() {
        let urls = [
            "https://google.com",
            "google.com",
            "google.com/",
            "google.com/route1/route-2",
            "google.com/route1/route-2?a=1&b=2",
            "google.com:8080/route1/route-2?a=1&b=2",
            "google.com:8080",
            "google.com:8080/",
            "localhost:8080/",
            "localhost:80",
        ];

        let outputs = urls
            .iter()
            .map(|url| alias_url_to_localhost(url))
            .collect::<Vec<_>>();

        assert!(outputs.iter().all(|output| output.is_none()));
    }
}
