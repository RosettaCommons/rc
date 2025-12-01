mod common;

use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;
use assert_fs::assert::PathAssert;
use predicates::prelude::*;
use std::fs;

common::engine_tests!(score);
common::engine_tests!(rosetta_score);

fn score(engine: &str) {
    use assert_fs::assert::PathAssert;
    use std::fs;

    let root = std::path::PathBuf::from("target/score").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
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
            "--container-engine",
            engine,
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
}

fn rosetta_score(engine: &str) {
    let root = std::path::PathBuf::from("target/rosetta_score").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
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
            "--container-engine",
            engine,
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
}
