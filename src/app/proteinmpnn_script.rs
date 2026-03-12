use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec},
    util::include_asset,
};

pub struct ProteinmpnnScript;
pub static PROTEINMPNN_SCRIPT: ProteinmpnnScript = ProteinmpnnScript;

fn map_input_and_output_options(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const OPTIONS: [&str; 2] = ["--input_path=", "--output_path="];

    fn make_absolute(working_dir: impl Into<Utf8PathBuf>, path_str: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from(path_str);
        if path.is_absolute() {
            path
        } else {
            working_dir.into().join(path)
        }
    }

    for option in OPTIONS {
        if !app_args.iter_mut().any(|arg| {
            if let Some(value) = arg.strip_prefix(option) {
                *arg = format!("{option}{}", make_absolute(working_dir, value));
                true
            } else {
                false
            }
        }) {
            app_args.push(format!("{option}/{working_dir}"));
        }
    }

    app_args
}

const SCRIPTS_WITH_INPUT_PATH_OPTION: &[&str] = &[
    "make_bias_AA.py",
    "make_pssm_input_dict.py",
    "parse_multiple_chains.py",
];

impl AppSpec for ProteinmpnnScript {
    fn container_image(&self) -> &'static str {
        "rosettacommons/proteinmpnn"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/proteinmpnn.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        assert!(
            !(app_args.is_empty() || app_args[0].starts_with("-")),
            "ProteinmpnnScript arguments must include a script name as first argument"
        );

        let script_have_input_path_option =
            !SCRIPTS_WITH_INPUT_PATH_OPTION.contains(&app_args[0].as_str());

        let mut app_args = if script_have_input_path_option {
            map_input_and_output_options(app_args, "/w".into())
        } else {
            app_args
        };

        app_args[0].insert_str(0, "/app/proteinmpnn/helper_scripts/");

        ContainerConfig::new(app_args)
            .working_dir("/w")
            .entrypoint("/app/proteinmpnn/.venv/bin/python")
    }

    fn native_spec(&self, mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        assert!(
            !(app_args.is_empty() || app_args[0].starts_with("-")),
            "ProteinmpnnScript arguments must include a script name as first argument"
        );

        app_args.insert(0, "python".into());
        app_args[1].insert_str(0, "helper_scripts/");

        let script_have_input_path_option =
            !SCRIPTS_WITH_INPUT_PATH_OPTION.contains(&app_args[0].as_str());

        let app_args = if script_have_input_path_option {
            map_input_and_output_options(app_args, working_dir)
        } else {
            app_args
        };

        let app_args = app_args
            .into_iter()
            .map(|arg| shell_escape::escape(arg.into()).into())
            .collect::<Vec<_>>();

        NativeRunSpec::new(app_args)
    }
}
