#![allow(non_snake_case)]

use insta::assert_yaml_snapshot as assert_snapshot;
// use treq::app::services::request::entities::METHODS;
// use treq::view::cli::commands::CliCommand;
use treq::view::cli::input::clap_definition::root_command;
use treq::view::cli::input::parser::parse_clap_input_to_commands;

#[test]
fn should_parse_to_normal_GET_submit_without_passing_method_as_subcommand_and_no_body() {
    let input = "treq url.com";
    let matches = root_command().get_matches_from(input.split_whitespace());
    let result = parse_clap_input_to_commands(matches).unwrap();

    assert!(result.len() == 1);
    assert_snapshot!(result);
}

#[test]
fn should_parse_to_normal_POST_submit_without_passing_method_as_subcommand_but_passing_some_body_data() {
    let input = "treq url.com Hello=World";
    let matches = root_command().get_matches_from(input.split_whitespace());
    let result = parse_clap_input_to_commands(matches).unwrap();

    assert!(result.len() == 1);
    assert_snapshot!(result);
}

#[test]
fn should_parse_all_methods_subcommands_to_normal_submits() {
    let all_methods = [
        "GET",
        "POST",
        "PUT",
        "DELETE",
        "HEAD",
        "PATCH",
    ];

    let inputs = all_methods
        .iter()
        .map(|method| format!("treq {} url.com", method))
        .collect::<Vec<_>>();

    inputs.iter().for_each(|input| {
        let matches = root_command().get_matches_from(input.split_whitespace());
        let result = parse_clap_input_to_commands(matches).unwrap();
        assert!(result.len() == 1);
        assert_snapshot!(result);
    });
}

#[test]
fn should_parse_same_way_with_or_without_protocol_in_url() {
    let input1 = "treq url.com";
    let input2 = "treq http://url.com";

    let matches1 = root_command().get_matches_from(input1.split_whitespace());
    let matches2 = root_command().get_matches_from(input2.split_whitespace());

    let result1 = parse_clap_input_to_commands(matches1).unwrap();
    let result2 = parse_clap_input_to_commands(matches2).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn should_error_if_no_input() {
    let input = "treq";
    let matches = root_command().get_matches_from(input.split_whitespace());
    let result = parse_clap_input_to_commands(matches);
    assert!(result.is_err());
}


// TODO Fix it in parser using regex validator
#[test]
fn should_error_if_subcommand_is_not_supported() {
    // let input = "treq unknown url.com";
    // let matches = root_command().get_matches_from(input.split_whitespace());
    // let result = parse_clap_input_to_commands(matches);
    // assert!(result.is_err());
}



// TODO: The body value has changed in ever iteration, once it's a HashMap and order of elements is
// changed. This test will be fixed

// #[test]
// fn should_persist_snapshot_to_edit() {
//     let input = "treq edit url.com";
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//     assert_snapshot!(result);
//
//     let input_with_body = "treq edit url.com Hello=World name=Bob";
//     let matches = root_command().get_matches_from(input_with_body.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//     assert_snapshot!(result);
//
//     let input_with_body_with_header = "treq edit url.com Auth:'Bearer token' Hello=World name=Bob";
//     let matches = root_command().get_matches_from(input_with_body_with_header.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//     assert_snapshot!(result);
//
//     let input_with_other_url_and_method = "treq edit url.com --method GET --url http://some_another_api.com";
//     let matches = root_command().get_matches_from(input_with_other_url_and_method.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//     assert_snapshot!(result);
// }
