use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use assert_fs::TempDir;

mod common;

common::engine_tests!(foundry);

fn foundry(engine: &str) {
    use assert_fs::assert::PathAssert;

    let root = std::path::PathBuf::from("target/foundry").join(engine);
    std::fs::create_dir_all(&root).expect("create engine testing dir");
    let work_dir = TempDir::new_in(root).expect("create temp dir");

    let json_rfd3_path = work_dir.join("test_rfd3.json");
    std::fs::write(json_rfd3_path, r#"{ "foundry": { "length": "10" } }"#)
        .expect("write test_rfd3.json");

    for (i, c) in [
        "rfd3 out_dir=/w/rfd3_out/ inputs=/w/test_rfd3.json skip_existing=False prevalidate_inputs=True ckpt_path=/weights/rfd3_latest.ckpt n_batches=1 diffusion_batch_size=1 inference_sampler.num_timesteps=10 low_memory_mode=True global_prefix=test_",
        "mpnn --structure_path /w/rfd3_out/test_foundry_0_model_0.cif.gz --checkpoint_path /weights/ligandmpnn_v_32_010_25.pt --is_legacy_weights True --model_type ligand_mpnn  --out_directory /w/mpnn_out",
        "rf3 fold inputs=/w/mpnn_out/test_foundry_0_model_0.cif_b0_d0.cif ckpt_path=/weights/rf3_foundry_01_24_latest_remapped.ckpt diffusion_batch_size=1 num_steps=10 out_dir=/w/rf3_out",
        ].iter()
        .enumerate() {
        let cmd = cargo_bin_cmd!()
            .args([
                "run",
                "--container-engine",
                engine,
                "-w",
                work_dir.path().to_str().unwrap(),
                "foundry",
            ])
            .args(c.split_ascii_whitespace())
            .unwrap();
        cmd.assert().success();

        use assert_fs::prelude::PathChild;

        work_dir
            .child(format!(".000{i}.rc.log"))
            .assert(predicates::path::exists());
    }
}
