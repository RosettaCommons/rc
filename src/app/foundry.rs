use std::path::Path;

use crate::{
    app::{ContainerRunSpec, NativeRunSpec},
    util::include_asset,
};

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    dbg!(&app_args);
    assert!(
        !(app_args.is_empty() || app_args[0].starts_with("-")),
        "Foundry arguments must include a protocol name as first argument"
    );

    ContainerRunSpec::new(
        "rosettacommons/foundry:weights",
        with_default_checkpoints(app_args, "/weights"),
    )
    .working_dir("/w")
}

pub fn native_spec(app_args: Vec<String>, _working_dir: &Path) -> NativeRunSpec {
    NativeRunSpec::new(
        include_asset!("pixi/foundry.toml"),
        with_default_checkpoints(app_args, "$FOUNDRY_CHECKPOINTS"),
    )
}

fn with_default_checkpoints(mut app_args: Vec<String>, weights_path: &str) -> Vec<String> {
    match app_args[0].as_str() {
        "mpnn" => {
            if !app_args.iter().any(|arg| arg == "--checkpoint_path") {
                app_args.extend([
                    "--checkpoint_path".into(),
                    format!("{weights_path}/ligandmpnn_v_32_010_25.pt"),
                ]);
            }
        }
        "rf3" => {
            if !app_args.iter().any(|arg| arg.starts_with("ckpt_path=")) {
                app_args.insert(
                    app_args.len().min(2),
                    format!("ckpt_path={weights_path}/rf3_foundry_01_24_latest_remapped.ckpt"),
                );
            }
        }
        "rfd3" => {
            if !app_args.iter().any(|arg| arg.starts_with("ckpt_path=")) {
                app_args.insert(1, format!("ckpt_path={weights_path}/rfd3_latest.ckpt"));
            }
        }
        _ => {}
    }
    app_args
}
