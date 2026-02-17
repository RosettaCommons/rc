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

    // match app_args[0].as_str() {
    //     "rfd3" => {
    //         // case 1
    //         // ckpt_path=
    //     }
    //     "mpnn" => {
    //         // case 2
    //         // --checkpoint_path
    //     }
    //     _ => {
    //         // default case
    //     }
    // }

    // ContainerRunSpec::with_prefixed_args(
    //     "rosettacommons/foundry:weights",
    //     std::iter::empty::<&str>(),
    //     app_args.clone(),
    // )

    ContainerRunSpec::new("rosettacommons/foundry:weights", app_args).working_dir("/w")
}

pub fn native_spec(app_args: Vec<String>, _working_dir: &Path) -> NativeRunSpec {
    NativeRunSpec::new(include_asset!("pixi/foundry.toml"), app_args)
}
