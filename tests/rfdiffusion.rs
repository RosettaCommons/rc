use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;

mod common;

common::engine_tests!(rfdiffusion);

fn rfdiffusion(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/rfdiffusion").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
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
            "--container-engine",
            engine,
            "-w",
            work_dir.path().to_str().unwrap(),
            "rfdiffusion",
            "inference.input_pdb=5tpn.pdb",
            "inference.num_designs=1",
            "contigmap.contigs=[10-40/A163-181/10-40]",
            //"contigmap.contigs=[10-10/A163-181/10-10]",
            //"contigmap.contigs=[100-100]",
            "diffuser.T=20",
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
}
