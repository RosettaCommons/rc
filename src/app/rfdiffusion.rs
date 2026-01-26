use std::path::{Path, PathBuf};

use crate::{
    app::{ContainerRunSpec, NativeRunSpec},
    util::include_asset,
};

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::with_prefixed_args(
        "rosettacommons/rfdiffusion",
        [
            "inference.output_prefix=/w/",
            "inference.model_directory_path=/app/RFdiffusion/models",
        ],
        app_args.clone(),
    )
    .scratch("/app/RFdiffusion/schedules")
    .working_dir("/w")
}

pub fn native_spec(mut app_args: Vec<String>, working_dir: &Path) -> NativeRunSpec {
    const INPUT_PDB_PREFIX: &str = "inference.input_pdb=";
    const OUTPUT_PREFIX: &str = "inference.output_prefix=";

    let mut output_prefix_present = false;
    for arg in &mut app_args {
        if arg.starts_with(INPUT_PDB_PREFIX) {
            let input_pdb = arg.split_at(INPUT_PDB_PREFIX.len()).1;

            let input_pdb_path = PathBuf::from(input_pdb);
            if !input_pdb_path.is_absolute() {
                let abs_input_pdb = working_dir.join(input_pdb_path);
                *arg = format!("inference.input_pdb={}", abs_input_pdb.to_str().unwrap());
            }
        } else if arg.starts_with(OUTPUT_PREFIX) {
            let output_prefix = arg.split_at(OUTPUT_PREFIX.len()).1;

            let output_prefix_path = PathBuf::from(output_prefix);
            if !output_prefix_path.is_absolute() {
                let abs_output_prefix = working_dir.join(output_prefix_path);
                *arg = format!(
                    "inference.output_prefix={}",
                    abs_output_prefix.to_str().unwrap()
                );
            }
            output_prefix_present = true;
        }
    }

    if !output_prefix_present {
        app_args.extend_from_slice(&[format!("{OUTPUT_PREFIX}{}/", working_dir.to_str().unwrap())]);
    }

    NativeRunSpec::new(include_asset!("pixi/rfdiffusion.toml"), app_args)
}
