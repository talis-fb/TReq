use regex::Regex;

pub fn regex_url() -> Regex {
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
}
