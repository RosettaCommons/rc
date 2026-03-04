use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;

mod common;

common::engine_tests!(proteinmpnn);

fn proteinmpnn(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/proteinmpnn").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "3htn";
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
            "proteinmpnn",
            "--pdb_path",
            &pdb_file,
            "--pdb_path_chains",
            "A B",
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    let o_pdb = work_dir.child("seqs/3htn.fa");
    o_pdb.assert(predicates::path::exists());

    // std::thread::sleep(std::time::Duration::from_secs(60));
}

common::engine_tests!(proteinmpnn_scripts);

fn proteinmpnn_scripts(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/proteinmpnn-scripts").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let pdb_id = "3htn";
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

    const OUTPUT_JSON: &str = "parsed_pdbs.jsonl";

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "--container-engine",
            engine,
            "-w",
            work_dir.path().to_str().unwrap(),
            "proteinmpnn-script",
            "parse_multiple_chains.py",
            &format!("--input_path={pdb_file}"),
            &format!("--output_path={OUTPUT_JSON}"),
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    for f in [".0000.rc.log", OUTPUT_JSON] {
        work_dir.child(f).assert(predicates::path::exists());
    }

    // std::thread::sleep(std::time::Duration::from_secs(60));
}
