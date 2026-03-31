use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec},
    util::include_asset,
};

pub struct Proteinmpnn;
pub static PROTEINMPNN: Proteinmpnn = Proteinmpnn;

/// Rewrites path arguments in `app_args` so that they are always absolute, and
/// ensures `--out_folder` is present.
///
/// Two options are recognised:
/// - `--pdb_path <path>` — path to the input PDB file or directory.
/// - `--out_folder <path>` — destination directory for ProteinMPNN output.
///
/// For each recognised option, if the supplied value is a relative path it is
/// resolved against `working_dir`.  If `--out_folder` is not present at all it
/// is appended with `working_dir` as its value, so downstream code can always
/// rely on an explicit output location.
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

        NativeRunSpec::new(app_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wd() -> &'static Utf8Path {
        Utf8Path::new("/work")
    }

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn injects_out_folder_when_absent() {
        let result = map_input_and_output_options(args(&["script.py"]), wd());
        assert_eq!(result, args(&["script.py", "--out_folder", "/work"]));
    }

    #[test]
    fn does_not_inject_out_folder_when_present_absolute() {
        let input = args(&["script.py", "--out_folder", "/custom/output"]);
        let result = map_input_and_output_options(input, wd());
        assert_eq!(
            result,
            args(&["script.py", "--out_folder", "/custom/output"])
        );
    }

    #[test]
    fn makes_out_folder_absolute_when_relative() {
        let input = args(&["script.py", "--out_folder", "results"]);
        let result = map_input_and_output_options(input, wd());
        assert_eq!(
            result,
            args(&["script.py", "--out_folder", "/work/results"])
        );
    }

    #[test]
    fn makes_pdb_path_absolute_when_relative() {
        let input = args(&["script.py", "--pdb_path", "input.pdb"]);
        let result = map_input_and_output_options(input, wd());
        assert_eq!(
            result,
            args(&[
                "script.py",
                "--pdb_path",
                "/work/input.pdb",
                "--out_folder",
                "/work"
            ])
        );
    }

    #[test]
    fn leaves_pdb_path_unchanged_when_absolute() {
        let input = args(&["script.py", "--pdb_path", "/data/input.pdb"]);
        let result = map_input_and_output_options(input, wd());
        assert_eq!(
            result,
            args(&[
                "script.py",
                "--pdb_path",
                "/data/input.pdb",
                "--out_folder",
                "/work"
            ])
        );
    }

    #[test]
    fn handles_both_options_together() {
        let input = args(&[
            "script.py",
            "--pdb_path",
            "input.pdb",
            "--out_folder",
            "out",
        ]);
        let result = map_input_and_output_options(input, wd());
        assert_eq!(
            result,
            args(&[
                "script.py",
                "--pdb_path",
                "/work/input.pdb",
                "--out_folder",
                "/work/out"
            ])
        );
    }

    #[test]
    fn empty_args_injects_out_folder() {
        let result = map_input_and_output_options(vec![], wd());
        assert_eq!(result, args(&["--out_folder", "/work"]));
    }
}
