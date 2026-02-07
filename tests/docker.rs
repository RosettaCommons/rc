mod common;

use std::format;

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

#[test]
fn mock_docker_rosetta_score() {
    let fixture = common::ContainerPathShim::new();
    let bin = fixture.install("docker");
    let log_file = bin.join("docker.log");

    let root = std::path::PathBuf::from("target/docker");
    std::fs::create_dir_all(&root).expect("create docker testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "-w",
            work_dir.path().to_str().unwrap(),
            "rosetta",
            "score",
        ])
        .envs(fixture.env_overrides())
        .env("TEST_INVOCATIONS_LOG", &log_file)
        .unwrap();
    cmd.assert().success();

    let log_contents = std::fs::read_to_string(&log_file).expect("Failed to read log file");

    let mut command_line_parts = vec![
        "docker run".into(),
        format!("--volume {}:/w", work_dir.path().to_str().unwrap()),
    ];

    #[cfg(unix)]
    command_line_parts.push(format!(
        "--user {}:{}",
        nix::unistd::getuid().as_raw(),
        nix::unistd::getgid().as_raw()
    ));

    command_line_parts.push("--workdir /w".into());

    for command_line_part in &command_line_parts {
        assert!(
            predicates::str::contains(command_line_part).eval(&log_contents),
            "Expected log to contain command line part:\n  {}\n\nActual log contents:\n{}",
            command_line_part,
            log_contents
        );
    }
}
