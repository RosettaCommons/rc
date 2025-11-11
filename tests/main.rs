use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn help() {
    let cmd = cargo_bin_cmd!().arg("--help").unwrap();
    cmd.assert().success();
}

#[test]
fn no_command_line_arguments() {
    cargo_bin_cmd!()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: No command specified"));
}

#[test]
fn no_run_no_app() {
    cargo_bin_cmd!()
        .arg("run")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:\n  <APP>\n",
        ));
}
