use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    spec::{AppSpec, ContainerConfig, NativeRunSpec},
    util::include_asset,
};

pub struct Rfdiffusion;
pub static RFDIFFUSION: Rfdiffusion = Rfdiffusion;

impl AppSpec for Rfdiffusion {
    fn container_image(&self) -> &'static str {
        "rosettacommons/rfdiffusion"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/rfdiffusion.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(
            [
                "inference.output_prefix=/w/",
                "inference.model_directory_path=/app/RFdiffusion/models",
            ],
            app_args.clone(),
        )
        .scratch("/app/RFdiffusion/schedules")
        .working_dir("/w")
    }

    fn native_spec(&self, mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        const INPUT_PDB_PREFIX: &str = "inference.input_pdb=";
        const OUTPUT_PREFIX: &str = "inference.output_prefix=";

        let mut output_prefix_present = false;

        let make_absolute = |path_str: &str| -> String {
            let path = Utf8PathBuf::from(path_str);
            if path.is_absolute() {
                path_str.to_string()
            } else {
                working_dir.join(path).into()
            }
        };

        for arg in &mut app_args {
            if let Some(input_pdb) = arg.strip_prefix(INPUT_PDB_PREFIX) {
                *arg = format!("{INPUT_PDB_PREFIX}{}", make_absolute(input_pdb));
            } else if let Some(output_prefix) = arg.strip_prefix(OUTPUT_PREFIX) {
                *arg = format!("{OUTPUT_PREFIX}{}", make_absolute(output_prefix));
                output_prefix_present = true;
            }
        }

        if !output_prefix_present {
            app_args.push(format!("{OUTPUT_PREFIX}{}/", working_dir));
        }

        let app_args = app_args
            .into_iter()
            .map(|arg| shell_escape::escape(arg.into()).into())
            .collect::<Vec<_>>();

        NativeRunSpec::new(self.pixi_recipe().unwrap(), app_args)
    }
}
