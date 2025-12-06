use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;

mod common;

common::engine_tests!(ligandmpnn);

fn ligandmpnn(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/proteinmpnn").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "1bc8";
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
            "ligandmpnn",
            "--model_type",
            "protein_mpnn",
            "--pdb_path",
            &pdb_file,
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    let o_pdb = work_dir.child("seqs/1bc8.fa");
    o_pdb.assert(predicates::path::exists());

    // std::thread::sleep(std::time::Duration::from_secs(60));
}
