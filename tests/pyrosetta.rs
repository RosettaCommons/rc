mod common;

use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;
use predicates::prelude::*;

common::engine_tests!(pyrosetta);

fn pyrosetta(engine: &str) {
    use assert_fs::assert::PathAssert;
    use std::fs;

    let root = std::path::PathBuf::from("target/score").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "1brs";
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
            "--container-engine",
            engine,
            "-w",
            work_dir.path().to_str().unwrap(),
            "pyrosetta", "-c",
            "import pyrosetta; pyrosetta.init(); pose=pyrosetta.pose_from_pdb('1brs.pdb'); print('1brs.pdb structure SCORE:', pyrosetta.get_score_function()(pose) )",
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    let log_file_name = work_dir.child(".0000.rc.log");

    log_file_name.assert(predicates::path::exists());

    let log = fs::read_to_string(&log_file_name).unwrap();

    for s in [
        "PyRosetta-4",
        "Created in JHU by Sergey Lyskov and PyRosetta Team",
        "core.init: Checking for fconfig files in pwd and ./rosetta/flags",
        "1brs.pdb structure SCORE: 255",
    ] {
        assert!(predicates::str::contains(s).eval(&log));
    }

    // std::thread::sleep(std::time::Duration::from_secs(60));
}
