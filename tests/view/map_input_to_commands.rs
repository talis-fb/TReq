#![allow(non_snake_case)]

use insta::assert_yaml_snapshot as assert_snapshot;
use treq::view::commands::ViewCommandChoice;
use treq::view::input::cli_definition::root_command;
use treq::view::input::cli_input::CliInput;
use treq::view::input_to_commands::map_input_to_commands;

fn process<'a>(input: impl IntoIterator<Item = &'a str>) -> anyhow::Result<Vec<ViewCommandChoice>> {
    let matches = root_command().get_matches_from(input);
    let inputs = CliInput::from_clap_matches(&matches)?;
    let commands_choices = map_input_to_commands(inputs)?;
    Ok(commands_choices)
}

#[test]
fn should_parse_to_normal_GET_submit_without_passing_method_as_subcommand_and_no_body() {
    let input = ["treq", "url.com"];
    let output = process(input).unwrap();
    assert!(output.len() == 1);
    assert_snapshot!(output);
}

#[test]
fn should_parse_to_normal_POST_submit_without_passing_method_as_subcommand_but_passing_some_body_data(
) {
    let input = ["treq", "url.com", "Hello=World"];
    let output = process(input).unwrap();
    assert!(output.len() == 1);
    assert_snapshot!(output);
}

#[test]
fn should_ignore_body_inputs_in_GET_request() {
    let input = ["treq", "GET", "url.com", "Hello=World"];
    let output = process(input).unwrap();
    assert!(output.len() == 1);
    assert_snapshot!(output);
}

#[test]
fn should_parse_all_methods_subcommands_to_normal_submits() {
    let all_methods = ["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"];

    let inputs = all_methods
        .into_iter()
        .map(|method| ["treq", method, "url.com"])
        .collect::<Vec<_>>();

    inputs.into_iter().for_each(|input| {
        let output = process(input).unwrap();
        debug_assert!(output.len() == 1, "{:?}", output);
        assert_snapshot!(output);
    });
}

#[test]
fn should_parse_same_way_with_or_without_protocol_in_url() {
    let input1 = ["treq", "url.com"];
    let input2 = ["treq", "http://url.com"];
    process(input1).unwrap();
    process(input2).unwrap();
}

#[test]
fn should_error_if_no_input() {
    let input = ["treq"];
    let output = process(input);
    assert!(output.is_err());
}

#[test]
fn should_raw_flag_work_equal_param_body_definition() {
    let input1 = ["treq", "POST", "url.com", "Hello=World"];
    let input2 = ["treq", "POST", "url.com", "--raw", r#"{"Hello":"World"}"#];
    let output1 = process(input1);
    let output2 = process(input2);

    debug_assert!(output1.is_ok(), "{:?}", output1);
    debug_assert!(output2.is_ok(), "{:?}", output2);
    assert_eq!(output1.unwrap(), output2.unwrap());
}

#[test]
fn should_merge_inputs_of_raw_flag_and_param_body() {
    let input = [
        "treq",
        "POST",
        "url.com",
        "--raw",
        r#"{"name":"Thales"}"#,
        "age=40",
        "job=Dev",
    ];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);
    assert_snapshot!(output.unwrap());
}

#[test]
fn should_set_query_params_by_request_items() {
    // Url final => url.com/?search=Rust&country=br
    let input = ["treq", "GET", "url.com", "search==Rust", "country==br"];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);
    assert_snapshot!(output.unwrap());
}

#[test]
fn should_execute_with_valid_urls() {
    let VALID_URLS = [
        "google.com",
        "google.com/",
        "google.com?",
        "google.com:81",
        "google.com:81/",
        "google.com/search/advanced",
        "google.com/search/advanced/",
        "google.com?search=Rust",
        "google.com?search=Rust&country=br",
        "google.com/search/advanced?name=john",
        "google.com/search/advanced/?name=john",
        "google.com/search/advanced?name=john&sort=true",
        "google.com/search/advanced?name=john&sort=true#landing-page",
        "google.com/search/advanced#landing-page",
        "google.com/search/advanced/#landing-page",
        "google.com#landing-page",
        "google.com/#landing-page",
        "maps.google.com/#landing-page",
        "subdomain.from-subdomain.maps.google.com/#",
        "subdomain.from-subdomain.maps.google.com:9999/#",
        "localhost",
        "localhost/",
        "localhost:8081/",
        "localhost:8081/api/v1/local",
    ];

    for url in VALID_URLS {
        let input = ["treq", "GET", url];
        let output = process(input);
        debug_assert!(output.is_ok(), "{:?}", output);
    }
}

// ---------------------------------
// Validators    -------------------
// ---------------------------------
#[test]
fn should_validate_alias_for_localhost_requests() {
    let input = ["treq", "GET", ":8080", "search==Rust", "country==br"];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);
    assert_snapshot!(output.unwrap());
}

#[test]
fn should_validate_alias_for_localhost_requests_with_manual_url() {
    let input = [
        "treq",
        "run",
        "some-request",
        "search==Rust",
        "--url",
        ":8080",
    ];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);
    assert_snapshot!(output.unwrap());
}

#[test]
fn should_validate_if_error_on_raw_value_as_not_map_and_body_insert() {
    // Ok
    let input = [
        "treq",
        "GET",
        "url.com",
        "--raw",
        "{\"Hello\":\"World\"}",
        "name=John",
    ];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);

    // Ok
    let input = ["treq", "GET", "url.com", "--raw", "{}", "name=John"];
    let output = process(input);
    debug_assert!(output.is_ok(), "{:?}", output);

    // Err
    let input = ["treq", "GET", "url.com", "--raw", "some-value", "name=John"];
    let output = process(input);
    debug_assert!(output.is_err(), "{:?}", output);
    debug_assert!(
        output
            .as_ref()
            .unwrap_err()
            .to_string()
            .contains("raw body"),
        "{:?}",
        output
    );

    // Err
    let input = [
        "treq",
        "GET",
        "url.com",
        "--raw",
        "[ \"Hello\",\"World\" ]",
        "name=John",
    ];
    let output = process(input);
    debug_assert!(output.is_err(), "{:?}", output);
    debug_assert!(
        output
            .as_ref()
            .unwrap_err()
            .to_string()
            .contains("raw body"),
        "{:?}",
        output
    );
}
