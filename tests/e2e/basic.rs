use std::str::FromStr;

use assert_cmd::Command;
use predicates::prelude::*;
use treq::app::services::request::entities::url::UrlInfo;

const DEFAULT_HTTPBIN_HOST: &str = "localhost:8888";

fn host() -> String {
    std::env::var("HTTPBIN_HOST").unwrap_or(DEFAULT_HTTPBIN_HOST.to_string())
}

#[test]
fn should_assert_process_do_not_return_error_with_a_basic_get_request() {
    let input = "treq GET google.com";
    let mut cmd = run_cmd(input);
    cmd.assert().success();
}

#[test]
fn should_assert_process_sucess_with_basic_requests() {
    let input = format!("treq POST {}/post", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = format!("treq DELETE {}/post", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert()
        .stdout(predicate::str::contains("Method Not Allowed"));
}

#[test]
fn should_assert_process_return_with_no_saved_requests_call() {
    // 1 - The request does not exist yet
    let input = "treq run unknown_req";
    let mut cmd = run_cmd(input);
    cmd.assert().failure();

    // 2 - The request is created
    let input = format!(
        "treq POST {}/post Hello=World --save-as unknown_req",
        host()
    );
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    // 3 - The request exists
    let input = "treq run unknown_req";
    let mut cmd = run_cmd(input);
    cmd.assert().success();
}

#[test]
fn should_merge_nested_body_values_with_saved() {
    let input = format!(
        "treq POST {}/post id=1 user[name]=John user[age]=30 user[address][city]=Tokio --save-as create-user",
        host()
    );
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run create-user Hello=World";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("Hello")
            .and(predicate::str::contains("World"))
            // previues saved nested values
            .and(predicate::str::contains("user"))
            .and(predicate::str::contains("address"))
            .and(predicate::str::contains("city"))
            .and(predicate::str::contains("Tokio"))
            .and(predicate::str::contains("age"))
            .and(predicate::str::contains("30")),
    );
}

#[test]
fn should_assert_list_saved_requests() {
    let requests_to_save = ["simple-get", "some-put", "a-great-post"];

    let inputs_to_create_requests = [
        format!("treq GET {}/get --save-as simple-get", host()),
        format!("treq PUT {}/put Hello=World --save-as some-put", host()),
        format!("treq POST {}/post user=John --save-as a-great-post", host()),
    ];

    inputs_to_create_requests.iter().for_each(|input| {
        let mut cmd = run_cmd(&input);
        cmd.assert().success();
    });

    let input = "treq ls";
    let mut cmd = run_cmd(input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains(requests_to_save[0])
            .and(predicate::str::contains(requests_to_save[1]))
            .and(predicate::str::contains(requests_to_save[2])),
    );
}

#[test]
fn should_inspect_command_show_info_about_a_saved_request() {
    // Setup
    let url = format!("{}/post", host());
    let url_data = UrlInfo::from_str(&url).unwrap();

    let input = format!("treq POST {} Hello=World --save-as some-cool-request", &url,);
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq inspect some-cool-request";
    let mut cmd = run_cmd(input);
    cmd.assert().success();
    cmd.assert().stdout(
        // Request data should be printed in stdout
        predicate::str::contains("some-cool-request")
            .and(predicate::str::contains("Hello"))
            .and(predicate::str::contains("World"))
            .and(predicate::str::contains("POST"))
            .and(predicate::str::contains(url_data.host.unwrap()))
            .and(predicate::str::contains("post")),
    );
}

#[test]
fn should_submit_save_edit_and_submit_corretly_in_sequence() {
    // Setup
    let input = format!("treq GET {}/get --save-as my-little-request", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = format!(
        "treq edit my-little-request --url {}/post --method POST",
        host()
    );
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run my-little-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("/post"));

    let input = "treq run my-little-request Hello=World --save";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("/post")
            .and(predicate::str::contains("Hello"))
            .and(predicate::str::contains("World")),
    );

    let input = "treq inspect my-little-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("post")
            .and(predicate::str::contains("Hello"))
            .and(predicate::str::contains("World")),
    );
}

#[test]
fn should_submit_save_edit_replacing_just_a_field_in_body() {
    let input = format!(
        "treq POST {}/post --save-as my-post-request email=john@gmail.com password=123",
        host()
    );
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run my-post-request name=John";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("/post")
            .and(predicate::str::contains("name"))
            .and(predicate::str::contains("John"))
            .and(predicate::str::contains("email"))
            .and(predicate::str::contains("john@gmail.com"))
            .and(predicate::str::contains("password"))
            .and(predicate::str::contains("123")),
    );

    let input = "treq run my-post-request password=456";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("/post")
            .and(predicate::str::contains("email"))
            .and(predicate::str::contains("john@gmail.com"))
            .and(predicate::str::contains("password"))
            .and(predicate::str::contains("456")),
    );
}

#[test]
fn should_save_query_params_without_delete_already_saved_url() {
    // Setup
    let input = format!("treq GET {}/get --save-as req-with-query-params", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run req-with-query-params search==Rust";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("/get")
            .and(predicate::str::contains("search"))
            .and(predicate::str::contains("Rust")),
    );

    let input = "treq run req-with-query-params --save search==Rust";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq inspect req-with-query-params";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("get")
            .and(predicate::str::contains("search"))
            .and(predicate::str::contains("Rust")),
    );
}

#[test]
fn should_save_request_as_another_file_if_used_only_run_with_save_as_command_to_another_name() {
    // Setup
    let input = format!("treq GET {}/get --save-as the-first-request", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run the-first-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run the-first-request --save-as the-second-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("/get"));
}

#[test]
fn should_overwrite_of_saved_url_work() {
    // Setup
    let input = format!(
        "treq GET {}/get --save-as req-with-some-query-params key1==value1",
        host()
    );
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run req-with-some-query-params key2==value2 --save";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("key1")
            .and(predicate::str::contains("value1"))
            .and(predicate::str::contains("key2"))
            .and(predicate::str::contains("value2")),
    );

    // Just to verify
    let input =
        "treq edit req-with-some-query-params --url :7777/patch --method PATCH key3==value3";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq inspect req-with-some-query-params";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("patch")
            .and(predicate::str::contains("7777"))
            .and(predicate::str::contains("key1"))
            .and(predicate::str::contains("value1"))
            .and(predicate::str::contains("key2"))
            .and(predicate::str::contains("value2"))
            .and(predicate::str::contains("key3"))
            .and(predicate::str::contains("value3")),
    );
}

#[test]
fn should_remove_requests() {
    // Setup
    let input = format!("treq GET {}/get --save-as my-request", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq remove my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().failure();

    let input = "treq inspect my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().failure();
}

#[test]
fn should_rename_requests() {
    // Setup
    let input = format!("treq GET {}/get --save-as request-to-be-renamed", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    // Before rename
    let input = "treq run request-to-be-renamed";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    let input = "treq run request-renamed";
    let mut cmd = run_cmd(&input);
    cmd.assert().failure();

    let input = "treq rename request-to-be-renamed request-renamed --no-confirm";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq rename request-to-be-renamed request-renamed --no-confirm";
    let mut cmd = run_cmd(&input);
    cmd.assert().failure();

    // After rename
    let input = "treq run request-to-be-renamed";
    let mut cmd = run_cmd(&input);
    cmd.assert().failure();
    let input = "treq run request-renamed";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
}

// ------------------
// UTILS
// ------------------
fn run_cmd(input: &str) -> Command {
    let bin_name = input.split_whitespace().next().unwrap();
    let args = input.split_whitespace().skip(1).collect::<Vec<_>>();

    let mut cmd = Command::cargo_bin(bin_name).unwrap();
    for arg in args {
        cmd.arg(arg);
    }

    cmd
}
