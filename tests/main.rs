mod common;

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
fn container_shim_test() {
    let fixture = common::ContainerPathShim::new();
    let bin = fixture.install_all();

    // Call each shim with dummy arguments and check the invocation logs
    for shim_name in common::ContainerPathShim::BIN_SHIMS {
        let log_file = bin.join(format!("{}.log", shim_name));
        let shim_path = bin.join(shim_name);

        for cmd in [
            std::process::Command::new("sh")
                .args([
                    "-c",
                    &format!(
                        "{} && {shim_name} arg1 arg2 --flag",
                        fixture.export_path_cmd()
                    ),
                ])
                .env("TEST_INVOCATIONS_LOG", &log_file),
            std::process::Command::new(shim_name)
                .args(["arg1", "arg2", "--flag"])
                .envs(fixture.env_overrides())
                .env("TEST_INVOCATIONS_LOG", &log_file),
        ] {
            let _r = cmd.output().expect("Failed to execute shim");
            // eprintln!("{_r:?}");

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
            std::fs::remove_file(&log_file).expect("Failed to delete log file");
        }
    }
    //std::thread::sleep(std::time::Duration::from_secs(60));
}
