mod fixtures;

use std::thread::sleep;

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

#[test]
fn shim_test() {
    let s = fixtures::ContainerPathShim::new();
    let bin = s.install_all();

    // Call each shim with dummy arguments and check the invocation logs
    for shim_name in fixtures::ContainerPathShim::BIN_SHIMS {
        let log_file = bin.join(format!("{}.log", shim_name));
        let shim_path = bin.join(shim_name);

        std::process::Command::new(&shim_path)
            .args(["arg1", "arg2", "--flag"])
            .env("TEST_INVOCATIONS_LOG", &log_file)
            .output()
            .expect("Failed to execute shim");

        // Read the log file and verify it contains the expected command line
        let log_contents = std::fs::read_to_string(&log_file).expect("Failed to read log file");

        let expected = format!("{} arg1 arg2 --flag", shim_path.display());
        assert!(
            log_contents.trim() == expected,
            "Log file for {} should contain '{}', but got '{}'",
            shim_name,
            expected,
            log_contents.trim()
        );
    }
    //sleep(std::time::Duration::from_secs(60));
}
