use assert_cmd::Command;
use predicates::prelude::*;

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
    let input = format!(
        "treq POST {}/post Hello=World --save-as some-cool-request",
        host()
    );
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
            .and(predicate::str::contains(format!("{}/post", host())))
            .and(predicate::str::contains("POST")),
    );
}

#[test]
fn should_submit_save_edit_and_submit_corretly_in_sequence() {
    // Setup
    let input = format!("treq GET {}/get --save-as my-request", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = format!("treq edit my-request --url {}/post --method POST", host());
    let mut cmd = run_cmd(&input);
    cmd.assert().success();

    let input = "treq run my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("/post"));

    let input = "treq run my-request Hello=World --save";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains("/post")
            .and(predicate::str::contains("Hello"))
            .and(predicate::str::contains("World")),
    );

    let input = "treq inspect my-request";
    let mut cmd = run_cmd(&input);
    cmd.assert().success();
    cmd.assert().stdout(
        predicate::str::contains(format!("{}/post", host()))
            .and(predicate::str::contains("Hello"))
            .and(predicate::str::contains("World")),
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
