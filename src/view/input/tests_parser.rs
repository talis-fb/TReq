// use std::collections::HashMap;
//
// use super::cli_definition::root_command;
// use super::parser::parse_clap_input_to_commands;
// use crate::app::services::request::entities::methods::METHODS;
// use crate::app::services::request::entities::requests::{OptionalRequestData, RequestData};
// use crate::view::commands::ViewCommandChoice;
//
// #[test]
// fn test_parse_all_methods_command() {
//     for method in [
//         METHODS::GET,
//         METHODS::POST,
//         METHODS::PUT,
//         METHODS::DELETE,
//         METHODS::HEAD,
//         METHODS::PATCH,
//     ] {
//         let input = root_command().get_matches_from(vec!["treq", &method.to_string(), "url.com"]);
//         assert_eq!(
//             parse_clap_input_to_commands(input).unwrap(),
//             Vec::from([ViewCommandChoice::SubmitRequest {
//                 request: RequestData::default()
//                     .with_method(method)
//                     .with_url("url.com")
//             }])
//         );
//     }
// }
//
// // TODO: Same of below but with header
//
// #[test]
// fn test_without_explicit_method_using_default_get() {
//     let input = root_command().get_matches_from(vec!["treq", "url.com"]);
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         vec![ViewCommandChoice::SubmitRequest {
//             request: RequestData::default()
//                 .with_method(METHODS::GET)
//                 .with_url("url.com")
//         }]
//     );
// }
//
// #[test]
// fn test_without_explicit_method_using_default_post() {
//     let input = vec!["treq", "url.com", "Hello=World"];
//
//     let arg_matches = root_command().get_matches_from(input);
//
//     assert_eq!(
//         parse_clap_input_to_commands(arg_matches).unwrap(),
//         vec![ViewCommandChoice::SubmitRequest {
//             request: RequestData::default()
//                 .with_method(METHODS::POST)
//                 .with_url("url.com")
//                 .with_body(r#"{"Hello":"World"}"#)
//         }]
//     );
// }
//
// #[test]
// fn test_parse_with_header_value() {
//     let input = root_command().get_matches_from(vec![
//         "treq",
//         "POST",
//         "http://httpbin.org/something",
//         "Auth:Value",
//     ]);
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         vec![ViewCommandChoice::SubmitRequest {
//             request: RequestData::default()
//                 .with_method(METHODS::POST)
//                 .with_headers([("Auth".into(), "Value".into())])
//                 .with_url("http://httpbin.org/something")
//         }]
//     );
// }
//
// #[test]
// fn test_parse_basic_post_with_save_as() {
//     let input = root_command().get_matches_from(vec![
//         "treq",
//         "POST",
//         "http://httpbin.org/user",
//         "--save-as",
//         "create_user",
//     ]);
//
//     let expected_request_data = OptionalRequestData {
//         url: Some("http://httpbin.org/user".into()),
//         method: Some(METHODS::POST),
//         headers: None,
//         body: None,
//     };
//
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         Vec::from([
//             ViewCommandChoice::SaveRequestWithBaseRequest {
//                 base_request_name: None,
//                 request_name: "create_user".to_string(),
//                 request_data: expected_request_data.clone(),
//             },
//             ViewCommandChoice::SubmitRequest {
//                 request: expected_request_data.to_request_data(),
//             }
//         ])
//     );
// }
//
// #[test]
// fn test_run_command() {
//     let input = root_command().get_matches_from(vec!["treq", "run", "create_user"]);
//
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         Vec::from([ViewCommandChoice::SubmitSavedRequest {
//             request_name: "create_user".into(),
//             request_data: OptionalRequestData::default(),
//         }])
//     );
// }
//
// #[test]
// fn test_run_command_with_additional_datas() {
//     let input = root_command().get_matches_from(vec!["treq", "run", "create_user", "Content:json"]);
//
//     let expected_request_data = OptionalRequestData {
//         headers: Some(HashMap::from([("Content".into(), "json".into())])),
//         url: None,
//         method: None,
//         body: None,
//     };
//
//     assert_eq!(
//         Vec::from([ViewCommandChoice::SubmitSavedRequest {
//             request_name: "create_user".into(),
//             request_data: expected_request_data
//         },]),
//         parse_clap_input_to_commands(input).unwrap(),
//     );
// }
//
// #[test]
// fn test_run_command_with_additional_datas_and_save_as() {
//     let input = root_command().get_matches_from(vec![
//         "treq",
//         "run",
//         "create_user",
//         "Content:json",
//         "--save-as",
//         "new_create_user",
//     ]);
//
//     let expected_request_data = OptionalRequestData {
//         url: None,
//         method: None,
//         headers: Some(HashMap::from([("Content".into(), "json".into())])),
//         body: None,
//     };
//
//     assert_eq!(
//         Vec::from([
//             ViewCommandChoice::SaveRequestWithBaseRequest {
//                 request_name: "new_create_user".into(),
//                 base_request_name: Some("create_user".into()),
//                 request_data: expected_request_data.clone(),
//             },
//             ViewCommandChoice::SubmitSavedRequest {
//                 request_name: "create_user".into(),
//                 request_data: expected_request_data
//             },
//         ]),
//         parse_clap_input_to_commands(input).unwrap(),
//     );
// }
//
// // test for "treq edit create_user --url url.com Content-type:something" command
// #[test]
// fn test_edit_command() {
//     let input = root_command().get_matches_from(vec![
//         "treq",
//         "edit",
//         "create_user",
//         "--url",
//         "url.com",
//         "Content-type:something",
//     ]);
//
//     assert_eq!(
//         Vec::from([ViewCommandChoice::SaveRequestWithBaseRequest {
//             request_name: "create_user".into(),
//             base_request_name: Some("create_user".into()),
//             request_data: OptionalRequestData {
//                 url: Some("url.com".into()),
//                 headers: Some(HashMap::from([("Content-type".into(), "something".into())])),
//                 method: None,
//                 body: None,
//             }
//         }]),
//         parse_clap_input_to_commands(input).unwrap(),
//     );
// }
//
// // test for "treq remove create_user" command
// #[test]
// fn test_remove_command() {
//     let input = root_command().get_matches_from(vec!["treq", "remove", "create_user"]);
//
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         Vec::from([ViewCommandChoice::RemoveSavedRequest {
//             request_name: "create_user".into()
//         }])
//     );
// }
//
// #[test]
// fn test_rename_command() {
//     let input = root_command().get_matches_from(vec!["treq", "rename", "create_user", "new_name"]);
//
//     assert_eq!(
//         parse_clap_input_to_commands(input).unwrap(),
//         Vec::from([ViewCommandChoice::RenameSavedRequest {
//             request_name: "create_user".into(),
//             new_name: "new_name".into()
//         }])
//     );
// }
//
// #[test]
// fn test_raw_flag_in_edit() {
//     let input = root_command().get_matches_from(vec![
//         "treq",
//         "edit",
//         "create_user",
//         "--raw",
//         "{ \"Hello\": \"World\" }",
//     ]);
//
//     assert_eq!(
//         Vec::from([ViewCommandChoice::SaveRequestWithBaseRequest {
//             request_name: "create_user".to_string(),
//             base_request_name: Some("create_user".to_string()),
//             request_data: OptionalRequestData {
//                 url: None,
//                 headers: None,
//                 method: None,
//                 body: Some("{ \"Hello\": \"World\" }".to_string()),
//             },
//         }]),
//         parse_clap_input_to_commands(input).unwrap(),
//     );
// }