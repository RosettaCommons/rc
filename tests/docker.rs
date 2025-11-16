#![cfg(feature = "test-docker")]

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;

mod fixtures;

#[test]
fn docker_rosetta_score() {
    let fixture = fixtures::ContainerPathShim::new();
    let bin = fixture.install("docker");

    let log_file = bin.join("docker.log");

    let cmd = cargo_bin_cmd!()
        .args(["run", "rosetta", "score"])
        .envs(fixture.env_overrides())
        .env("TEST_INVOCATIONS_LOG", &log_file)
        .unwrap();
    cmd.assert().success();
    
    let log_contents = std::fs::read_to_string(&log_file).expect("Failed to read log file");

}
