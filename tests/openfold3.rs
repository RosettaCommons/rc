use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;
use predicates::prelude::*;

mod common;

// common::engine_tests!(openfold3_predict; engines(docker, apptainer, singularity));
common::engine_tests!(openfold3_predict; engines(none));

fn openfold3_predict(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/openfold3").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let query_json = r#"{
	"queries": {
		"ubiquitin": {
			"chains": [
				{
					"molecule_type": "protein",
					"chain_ids": [
						"A"
					],
					"sequence": "MQIFVKTLTGKTITLEVEPSDTIENVKAKIQDKEGIPPDQQRLIFAGKQLEDGRTLSDYNIQKESTLHLVLRLRGG"
				}
			]
		}
	}
}"#;

    let query_json_path = work_dir.join("test_query.json");
    std::fs::write(&query_json_path, query_json).expect("write test_query.json");

    let cmd = cargo_bin_cmd!()
        .args([
            "run",
            "--container-engine",
            engine,
            "-w",
            work_dir.path().to_str().unwrap(),
            "openfold3",
            "test_query.json", //query_json_path.to_str().unwrap(),
        ])
        .unwrap();
    cmd.assert().success();

    use assert_fs::prelude::PathChild;

    work_dir
        .child(".0000.rc.log")
        .assert(predicates::path::exists());

    work_dir
        .child("ubiquitin")
        .assert(predicates::path::exists());

    work_dir
        .child("summary.txt")
        .assert(predicates::path::exists());

    let summary =
        std::fs::read_to_string(work_dir.child("summary.txt").path()).expect("read summary.txt");

    assert!(
        predicates::str::contains("- Failed Queries:      0").eval(&summary),
        "summary.txt does not contain expected string. Contents:\n{summary}"
    );
}
