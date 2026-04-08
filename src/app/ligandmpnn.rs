use camino::Utf8Path;

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec, make_absolute},
    util::include_asset,
};

pub struct Ligandmpnn;
pub static LIGANDMPNN: Ligandmpnn = Ligandmpnn;

/// Resolves `--pdb_path` and `--out_folder` argument values to absolute paths and ensures
/// `--out_folder` is always present in the returned argument list.
///
/// For each of `--pdb_path` and `--out_folder`, if the option appears in `app_args` its
/// immediately-following value is made absolute: relative paths are joined onto `working_dir`,
/// while already-absolute paths are left unchanged.
///
/// If `--out_folder` is not present at all, it is appended as `--out_folder <working_dir>` so
/// that LigandMPNN always writes its output to a known location.
fn map_input_and_output_options(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const INPUT_OPTION: &str = "--pdb_path";
    const OUTPUT_OPTION: &str = "--out_folder";

    for option in [INPUT_OPTION, OUTPUT_OPTION] {
        if let Some(i) = app_args.iter().position(|a| a == option)
            && let Some(val) = app_args.get_mut(i + 1)
        {
            *val = make_absolute(working_dir, val).into();
        }
    }

    if !app_args.iter().any(|a| a == OUTPUT_OPTION) {
        app_args.extend([OUTPUT_OPTION.into(), working_dir.to_string()]);
    }
    app_args
}

impl AppSpec for Ligandmpnn {
    fn container_image(&self) -> &'static str {
        "rosettacommons/ligandmpnn"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/ligandmpnn.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(
            [
                "--out_folder=/w",
                "--checkpoint_protein_mpnn",
                "/app/ligandmpnn/model_params/proteinmpnn_v_48_020.pt",
            ],
            app_args,
        )
        .working_dir("/w")

        // app_args.splice(
        //     0..0,
        //     [
        //         "--out_folder=/w".into(),
        //         "--checkpoint_protein_mpnn".into(),
        //         "/app/ligandmpnn/model_params/proteinmpnn_v_48_020.pt".into(),
        //     ],
        // );
        // RunSpec::new("rosettacommons/ligandmpnn", app_args).working_dir("/w")
    }

    fn native_spec(&self, app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        let app_args = app_args
            .into_iter()
            .map(|arg| shell_escape::escape(arg.into()).into())
            .collect::<Vec<_>>();

        let mut app_args = map_input_and_output_options(app_args, working_dir);

        app_args.splice(
            0..0,
            [
                "--checkpoint_protein_mpnn".into(),
                "$LIGANDMPNN_WEIGHTS/proteinmpnn_v_48_020.pt".into(),
            ],
        );

        NativeRunSpec::new(app_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wd() -> &'static Utf8Path {
        Utf8Path::new("/work/dir")
    }

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // ------------------------------------------------------------------
    // --out_folder injection
    // ------------------------------------------------------------------

    #[test]
    fn appends_out_folder_when_absent() {
        let result = map_input_and_output_options(args(&["run.py"]), wd());
        assert_eq!(
            result,
            args(&["run.py", "--out_folder", "/work/dir"]),
            "should append --out_folder <working_dir> when the option is missing"
        );
    }

    #[test]
    fn appends_out_folder_to_empty_args() {
        let result = map_input_and_output_options(vec![], wd());
        assert_eq!(result, args(&["--out_folder", "/work/dir"]));
    }

    // ------------------------------------------------------------------
    // --out_folder path resolution
    // ------------------------------------------------------------------

    #[test]
    fn makes_relative_out_folder_absolute() {
        let result =
            map_input_and_output_options(args(&["run.py", "--out_folder", "results"]), wd());
        assert_eq!(
            result,
            args(&["run.py", "--out_folder", "/work/dir/results"]),
        );
    }

    #[test]
    fn leaves_absolute_out_folder_unchanged() {
        let result = map_input_and_output_options(
            args(&["run.py", "--out_folder", "/absolute/results"]),
            wd(),
        );
        assert_eq!(
            result,
            args(&["run.py", "--out_folder", "/absolute/results"]),
        );
    }

    #[test]
    fn does_not_duplicate_out_folder_when_already_present() {
        let result =
            map_input_and_output_options(args(&["run.py", "--out_folder", "results"]), wd());
        let count = result
            .iter()
            .filter(|a| a.as_str() == "--out_folder")
            .count();
        assert_eq!(count, 1, "--out_folder must appear exactly once");
    }

    // ------------------------------------------------------------------
    // --pdb_path resolution
    // ------------------------------------------------------------------

    #[test]
    fn makes_relative_pdb_path_absolute() {
        let result =
            map_input_and_output_options(args(&["run.py", "--pdb_path", "input.pdb"]), wd());
        // --out_folder is also appended since it was absent
        assert_eq!(
            result,
            args(&[
                "run.py",
                "--pdb_path",
                "/work/dir/input.pdb",
                "--out_folder",
                "/work/dir",
            ]),
        );
    }

    #[test]
    fn leaves_absolute_pdb_path_unchanged() {
        let result =
            map_input_and_output_options(args(&["run.py", "--pdb_path", "/data/input.pdb"]), wd());
        assert!(
            result.contains(&"/data/input.pdb".to_string()),
            "absolute --pdb_path should not be modified"
        );
    }

    // ------------------------------------------------------------------
    // Both options together
    // ------------------------------------------------------------------

    #[test]
    fn handles_both_options_with_relative_paths() {
        let result = map_input_and_output_options(
            args(&[
                "run.py",
                "--pdb_path",
                "inputs/my.pdb",
                "--out_folder",
                "outputs",
            ]),
            wd(),
        );
        assert_eq!(
            result,
            args(&[
                "run.py",
                "--pdb_path",
                "/work/dir/inputs/my.pdb",
                "--out_folder",
                "/work/dir/outputs",
            ]),
        );
    }

    #[test]
    fn handles_both_options_with_absolute_paths() {
        let result = map_input_and_output_options(
            args(&[
                "run.py",
                "--pdb_path",
                "/abs/inputs/my.pdb",
                "--out_folder",
                "/abs/outputs",
            ]),
            wd(),
        );
        assert_eq!(
            result,
            args(&[
                "run.py",
                "--pdb_path",
                "/abs/inputs/my.pdb",
                "--out_folder",
                "/abs/outputs",
            ]),
        );
    }

    // ------------------------------------------------------------------
    // Unrelated arguments are preserved as-is
    // ------------------------------------------------------------------

    #[test]
    fn preserves_unrelated_args() {
        let result = map_input_and_output_options(
            args(&[
                "run.py",
                "--num_seq_per_target",
                "10",
                "--pdb_path",
                "input.pdb",
            ]),
            wd(),
        );
        assert_eq!(result[1], "--num_seq_per_target");
        assert_eq!(result[2], "10");
    }
}
