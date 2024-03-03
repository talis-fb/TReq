use std::sync::OnceLock;

use regex::Regex;

static URL_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn complete_url() -> &'static Regex {
    URL_REGEX.get_or_init(|| {
        Regex::new(
            &[
                r"^",
                r"((?<protocol>https?):\/\/)?",
                r"(?<host>[a-zA-Z0-9._@+=-]+)",
                r"(\:(?<port>[0-9]{1,6}))?",
                r"(\/(?<routes>[a-zA-Z0-9._@=+\/-]+))?",
                r"(\/)?", // For accepting optional '/' at end of url
                r"(\?)?", // For accepting optional '?' at end of url
                r"(\?(?<query_params>[a-zA-Z0-9._@=+\&=-]+))?",
                r"(\#(?<anchor>[a-zA-Z0-9._-]+))?",
                r"$",
            ]
            .join("")
            .to_string(),
        )
        .unwrap()
    })
}

pub mod request_items {
    use super::*;

    static BODY_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn body_value() -> &'static Regex {
        BODY_VALUE_REGEX.get_or_init(|| Regex::new(r"^(?<key>[ -~]*[^:=])=(?<value>[ -~]+)$").unwrap())
    }

    static NESTED_BODY_KEYS_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn nested_body_keys() -> &'static Regex {
        NESTED_BODY_KEYS_REGEX.get_or_init(|| {
            Regex::new(r"^(?<root_key>[^\[\]]+)(?<sub_keys>(\[([^\[\]]+)\])+)$").unwrap()
        })
    }

    static HEADER_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn header_value() -> &'static Regex {
        HEADER_VALUE_REGEX.get_or_init(|| Regex::new(r"^(?<key>[ -~]+):(?<value>[ -~]+)$").unwrap())
    }

    static QUERY_PARAM_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn query_param_value() -> &'static Regex {
        QUERY_PARAM_VALUE_REGEX
            .get_or_init(|| Regex::new(r"^(?<key>[ -~]+)==(?<value>[ -~]+)$").unwrap())
    }

    static NON_STRING_BODY_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn non_string_body_value() -> &'static Regex {
        NON_STRING_BODY_VALUE_REGEX
            .get_or_init(|| Regex::new(r"^(?<key>[ -~]+):=(?<value>[ -~]+)$").unwrap())
    }

    static ENCLOSED_BY_SINGLE_QUOTE_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn enclosed_by_single_quote_value() -> &'static Regex {
        ENCLOSED_BY_SINGLE_QUOTE_VALUE_REGEX
            .get_or_init(|| Regex::new(r"^'(?<value>[ -~]*)'$").unwrap())
    }

    static ENCLOSED_BY_DOUBLE_QUOTE_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    pub fn enclosed_by_double_quote_value() -> &'static Regex {
        ENCLOSED_BY_DOUBLE_QUOTE_VALUE_REGEX
            .get_or_init(|| Regex::new(r#"^"(?<value>[ -~]*)"$"#).unwrap())
    }
}
