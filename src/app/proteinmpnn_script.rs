use camino::Utf8Path;

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec, make_absolute},
    util::include_asset,
};

pub struct ProteinmpnnScript;
pub static PROTEINMPNN_SCRIPT: ProteinmpnnScript = ProteinmpnnScript;

/// Ensures that `--input_path=` and `--output_path=` arguments in `app_args` are absolute paths.
///
/// For each of the two options the function behaves as follows:
///
/// * **Option already present** – the path value is resolved against `working_dir` when it is
///   relative; absolute values are left unchanged.
/// * **Option absent** – a default argument of the form `{option}/{working_dir}` is appended,
///   producing an absolute path regardless of whether `working_dir` itself is relative or
///   absolute.
///
/// All other arguments in `app_args` are passed through untouched.
fn map_input_and_output_options(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const OPTIONS: [&str; 2] = ["--input_path=", "--output_path="];

    for option in OPTIONS {
        if !app_args.iter_mut().any(|arg| {
            if let Some(value) = arg.strip_prefix(option) {
                *arg = format!("{option}{}", make_absolute(working_dir, value));
                true
            } else {
                false
            }
        }) {
            app_args.push(format!("{option}{}", working_dir));
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
            SCRIPTS_WITH_INPUT_PATH_OPTION.contains(&app_args[0].as_str());

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

        let script_have_input_path_option =
            SCRIPTS_WITH_INPUT_PATH_OPTION.contains(&app_args[0].as_str());

        app_args.insert(0, "python".into());
        app_args[1].insert_str(0, "helper_scripts/");

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

#[cfg(test)]
mod tests {
    use super::*;

    fn wd(s: &str) -> &Utf8Path {
        Utf8Path::new(s)
    }

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // ── input_path ────────────────────────────────────────────────────────────

    #[test]
    fn relative_input_path_is_made_absolute() {
        let result = map_input_and_output_options(
            args(&["--input_path=data/pdbs", "--output_path=/out"]),
            wd("/work"),
        );
        assert!(
            result.contains(&"--input_path=/work/data/pdbs".to_string()),
            "got: {result:?}"
        );
    }

    #[test]
    fn absolute_input_path_is_unchanged() {
        let result = map_input_and_output_options(
            args(&["--input_path=/abs/input", "--output_path=/out"]),
            wd("/work"),
        );
        assert!(
            result.contains(&"--input_path=/abs/input".to_string()),
            "got: {result:?}"
        );
    }

    #[test]
    fn missing_input_path_appends_default() {
        let result = map_input_and_output_options(args(&["--output_path=/out"]), wd("/work"));
        assert!(
            result.contains(&"--input_path=/work".to_string()),
            "got: {result:?}"
        );
    }

    // ── output_path ───────────────────────────────────────────────────────────

    #[test]
    fn relative_output_path_is_made_absolute() {
        let result = map_input_and_output_options(
            args(&["--input_path=/in", "--output_path=results/out"]),
            wd("/work"),
        );
        assert!(
            result.contains(&"--output_path=/work/results/out".to_string()),
            "got: {result:?}"
        );
    }

    #[test]
    fn absolute_output_path_is_unchanged() {
        let result = map_input_and_output_options(
            args(&["--input_path=/in", "--output_path=/abs/output"]),
            wd("/work"),
        );
        assert!(
            result.contains(&"--output_path=/abs/output".to_string()),
            "got: {result:?}"
        );
    }

    #[test]
    fn missing_output_path_appends_default() {
        let result = map_input_and_output_options(args(&["--input_path=/in"]), wd("/work"));
        assert!(
            result.contains(&"--output_path=/work".to_string()),
            "got: {result:?}"
        );
    }

    // ── both options ──────────────────────────────────────────────────────────

    #[test]
    fn both_options_missing_appends_both_defaults() {
        let result = map_input_and_output_options(args(&[]), wd("/work"));
        assert!(
            result.contains(&"--input_path=/work".to_string()),
            "missing --input_path default; got: {result:?}"
        );
        assert!(
            result.contains(&"--output_path=/work".to_string()),
            "missing --output_path default; got: {result:?}"
        );
    }

    #[test]
    fn both_relative_options_are_resolved() {
        let result = map_input_and_output_options(
            args(&["--input_path=in", "--output_path=out"]),
            wd("/work"),
        );
        assert_eq!(
            result,
            args(&["--input_path=/work/in", "--output_path=/work/out"])
        );
    }

    // ── unrelated args ────────────────────────────────────────────────────────

    #[test]
    fn unrelated_args_are_preserved() {
        let result = map_input_and_output_options(
            args(&[
                "--input_path=/in",
                "--output_path=/out",
                "--num_seq_per_target=8",
                "--sampling_temp=0.1",
            ]),
            wd("/work"),
        );
        assert!(result.contains(&"--num_seq_per_target=8".to_string()));
        assert!(result.contains(&"--sampling_temp=0.1".to_string()));
    }

    #[test]
    fn unrelated_args_are_preserved_when_defaults_are_appended() {
        let result = map_input_and_output_options(args(&["--num_seq_per_target=8"]), wd("/work"));
        assert!(result.contains(&"--num_seq_per_target=8".to_string()));
        assert!(result.contains(&"--input_path=/work".to_string()));
        assert!(result.contains(&"--output_path=/work".to_string()));
    }
}
