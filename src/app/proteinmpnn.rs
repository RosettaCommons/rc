use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    spec::{AppSpec, ContainerConfig, NativeRunSpec},
    util::include_asset,
};

pub struct Proteinmpnn;
pub static PROTEINMPNN: Proteinmpnn = Proteinmpnn;

fn map_input_and_output_options(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const OPTIONS: [&str; 2] = ["--pdb_path", "--out_folder"];
    const OUTPUT_OPTION: &str = OPTIONS[1];

    let mut output_option_present = false;

    fn make_absolute(working_dir: &Utf8Path, path_str: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from(path_str);
        if path.is_absolute() {
            path
        } else {
            working_dir.join(path)
        }
    }

    for i in 1..app_args.len() {
        for option in OPTIONS {
            if app_args[i - 1] == option {
                app_args[i] = make_absolute(working_dir, &app_args[i]).into();
                if option == OUTPUT_OPTION {
                    output_option_present = true;
                }
                break;
            }
        }
    }

    if !output_option_present {
        app_args.extend([OUTPUT_OPTION.into(), working_dir.to_string()]);
    }
    app_args
}

impl AppSpec for Proteinmpnn {
    fn container_image(&self) -> &'static str {
        "rosettacommons/proteinmpnn"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/proteinmpnn.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(["--out_folder=/w"], app_args).working_dir("/w")
    }

    fn native_spec(&self, mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        app_args.splice(0..0, ["python".into(), "protein_mpnn_run.py".into()]);

        let app_args = map_input_and_output_options(app_args, working_dir);

        let app_args = app_args
            .into_iter()
            .map(|arg| shell_escape::escape(arg.into()).into())
            .collect::<Vec<_>>();

        NativeRunSpec::new(self.pixi_recipe().unwrap(), app_args)
    }
}
