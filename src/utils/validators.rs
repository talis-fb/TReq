use regex::Regex;

use super::regexes::regex_url;

pub fn is_url(url: &str) -> bool {
    regex_url().is_match(url)
}

pub fn is_url_with_localhost_alias(url: &str) -> bool {
    let re = Regex::new(r"^:\/[ -~]+$").unwrap();
    re.is_match(url)
}

pub fn is_header_input(url: &str) -> bool {
    let re = Regex::new(r"^[ -~]+:[ -~]+$").unwrap();
    re.is_match(url)
}

pub fn is_query_param_input(url: &str) -> bool {
    let re = Regex::new(r"^[ -~]+==[ -~]+$").unwrap();
    re.is_match(url)
}

pub fn is_body_data_input(url: &str) -> bool {
    let re = Regex::new(r"^[ -~]+=[ -~]+$").unwrap();
    re.is_match(url)
}
