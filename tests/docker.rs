use std::format;

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use assert_fs::TempDir;

use predicates::prelude::*;

mod fixtures;

#[test]
fn mock_docker_rosetta_score() {
    let fixture = fixtures::ContainerPathShim::new();
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
        users::get_current_uid(),
        users::get_current_gid()
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

#[test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
fn docker_rosetta_score() {
    use assert_fs::assert::PathAssert;
    use std::fs;

    let root = std::path::PathBuf::from("target/docker");
    std::fs::create_dir_all(&root).expect("create docker testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "1brs";
    let pdb_file = pdb_id.to_string() + ".pdb";
    let score_file_name = "output.score";

    let pdb_path = work_dir.path().join(&pdb_file);
    std::fs::write(
        pdb_path,
        reqwest::blocking::get(format!("https://files.rcsb.org/download/{pdb_file}"))
            .unwrap()
            .bytes()
            .unwrap(),
    )
    .unwrap();

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "-w",
            work_dir.path().to_str().unwrap(),
            "rosetta",
            "score",
            "-out:file:scorefile",
            score_file_name,
            "-in:file:s",
            &pdb_file,
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    let score_file = work_dir.child(score_file_name);
    score_file.assert(predicates::path::exists());

    let score = fs::read_to_string(&score_file).unwrap();

    assert!(predicates::str::contains("SCORE:").eval(&score));
    assert!(predicates::str::contains(format!("{pdb_id}_0001")).eval(&score));

    // std::thread::sleep(std::time::Duration::from_secs(60));
    fixtures::docker_clear_cache();
}

#[test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
fn docker_score() {
    use assert_fs::assert::PathAssert;
    use std::fs;

    let root = std::path::PathBuf::from("target/docker");
    std::fs::create_dir_all(&root).expect("create docker testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "1brs";
    let pdb_file = pdb_id.to_string() + ".pdb";
    let score_file_name = "output.score";

    let pdb_path = work_dir.path().join(&pdb_file);
    std::fs::write(
        pdb_path,
        reqwest::blocking::get(format!("https://files.rcsb.org/download/{pdb_file}"))
            .unwrap()
            .bytes()
            .unwrap(),
    )
    .unwrap();

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "-w",
            work_dir.path().to_str().unwrap(),
            "score",
            "-out:file:scorefile",
            score_file_name,
            "-in:file:s",
            &pdb_file,
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    let score_file = work_dir.child(score_file_name);
    score_file.assert(predicates::path::exists());

    let score = fs::read_to_string(&score_file).unwrap();

    assert!(predicates::str::contains("SCORE:").eval(&score));
    assert!(predicates::str::contains(format!("{pdb_id}_0001")).eval(&score));

    // std::thread::sleep(std::time::Duration::from_secs(60));
    fixtures::docker_clear_cache();
}

#[test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
fn docker_rfdiffusion() {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/docker");
    std::fs::create_dir_all(&root).expect("create docker testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "5tpn";
    let pdb_file = pdb_id.to_string() + ".pdb";

    let pdb_path = work_dir.path().join(&pdb_file);
    std::fs::write(
        pdb_path,
        reqwest::blocking::get(format!("https://files.rcsb.org/download/{pdb_file}"))
            .unwrap()
            .bytes()
            .unwrap(),
    )
    .unwrap();

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "-w",
            work_dir.path().to_str().unwrap(),
            "rfdiffusion",
            "inference.input_pdb=5tpn.pdb",
            "inference.num_designs=1",
            "contigmap.contigs=[10-40/A163-181/10-40]",
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    let o_pdb = work_dir.child("_0.pdb");
    o_pdb.assert(predicates::path::exists());

    let o_trb = work_dir.child("_0.trb");
    o_trb.assert(predicates::path::exists());

    // let score_file = work_dir.child(score_file_name);
    // score_file.assert(predicates::path::exists());

    // let score = fs::read_to_string(&score_file).unwrap();

    // assert!(predicates::str::contains("SCORE:").eval(&score));
    // assert!(predicates::str::contains(format!("{pdb_id}_0001")).eval(&score));

    // std::thread::sleep(std::time::Duration::from_secs(60));
    fixtures::docker_clear_cache();
}
