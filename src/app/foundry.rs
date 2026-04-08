use camino::Utf8Path;

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec, make_absolute},
    util::include_asset,
};

pub struct Foundry;
pub static FOUNDRY: Foundry = Foundry;

/// Injects a default checkpoint path into `app_args` when the caller has not
/// supplied one explicitly.
///
/// Each protocol uses a different convention for specifying its checkpoint:
///
/// | Protocol | Flag / prefix           | Insertion point                 |
/// |----------|-------------------------|---------------------------------|
/// | `mpnn`   | `--checkpoint_path <p>` | appended at the end             |
/// | `rf3`    | `ckpt_path=<p>`         | inserted at index `min(len, 2)` |
/// | `rfd3`   | `ckpt_path=<p>`         | inserted at index 1             |
///
/// If the relevant flag or prefix is already present the args are returned
/// unchanged, so calling this function is always safe even when the user has
/// provided their own checkpoint.
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

fn map_inputs_and_out_dir(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const PREFIXS: [&str; 2] = ["inputs=", "out_dir="];
    const OUTPUT_PREFIX: &str = PREFIXS[1];

    let mut output_prefix_present = false;

    for arg in &mut app_args {
        for prefix in PREFIXS {
            if let Some(value) = arg.strip_prefix(prefix) {
                *arg = format!("{prefix}{}", make_absolute(working_dir, value));
                if prefix == OUTPUT_PREFIX {
                    output_prefix_present = true;
                }
                break;
            }
        }
    }

    if !output_prefix_present {
        app_args.push(format!("{OUTPUT_PREFIX}{}/", working_dir));
    }
    app_args
}

fn map_input_and_output_options(mut app_args: Vec<String>, working_dir: &Utf8Path) -> Vec<String> {
    const INPUT_OPTION: &str = "--structure_path";
    const OUTPUT_OPTION: &str = "--out_directory";

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

impl AppSpec for Foundry {
    fn container_image(&self) -> &'static str {
        "rosettacommons/foundry:weights"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/foundry.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        assert!(
            !(app_args.is_empty() || app_args[0].starts_with("-")),
            "Foundry arguments must include a protocol name as first argument"
        );

        ContainerConfig::new(with_default_checkpoints(app_args, "/weights")).working_dir("/w")
    }

    fn native_spec(&self, app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        assert!(
            !(app_args.is_empty() || app_args[0].starts_with("-")),
            "Foundry arguments must include a protocol name as first argument"
        );

        let app_args = app_args
            .into_iter()
            .map(|a| shell_escape::escape(a.into()).into())
            .collect::<Vec<_>>();

        let app_args = with_default_checkpoints(app_args, "$FOUNDRY_CHECKPOINTS");

        let app_args = match app_args[0].as_str() {
            "mpnn" => map_input_and_output_options(app_args, working_dir),
            "rf3" | "rfd3" => map_inputs_and_out_dir(app_args, working_dir),
            _ => app_args,
        };

        // for arg in &mut app_args {
        //     *arg = shell_escape::escape(arg.as_str().into()).to_string();
        // }
        // app_args.insert(0, format!("cd {} && ", working_dir.to_string_lossy()));

        NativeRunSpec::new(app_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEIGHTS: &str = "/weights";

    // ── mpnn ────────────────────────────────────────────────────────────────

    #[test]
    fn mpnn_adds_default_checkpoint_when_absent() {
        let args = vec!["mpnn".into(), "--some_flag".into(), "value".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "mpnn",
                "--some_flag",
                "value",
                "--checkpoint_path",
                "/weights/ligandmpnn_v_32_010_25.pt",
            ]
        );
    }

    #[test]
    fn mpnn_does_not_overwrite_existing_checkpoint() {
        let args = vec![
            "mpnn".into(),
            "--checkpoint_path".into(),
            "/custom/model.pt".into(),
        ];
        let result = with_default_checkpoints(args.clone(), WEIGHTS);
        assert_eq!(result, args);
    }

    #[test]
    fn mpnn_only_arg_gets_checkpoint_appended() {
        let args = vec!["mpnn".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "mpnn",
                "--checkpoint_path",
                "/weights/ligandmpnn_v_32_010_25.pt",
            ]
        );
    }

    // ── rf3 ─────────────────────────────────────────────────────────────────

    #[test]
    fn rf3_inserts_default_ckpt_path_at_position_2_when_enough_args() {
        // len=3 before insert → insert at min(3,2)=2
        let args = vec!["rf3".into(), "arg1".into(), "arg2".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "rf3",
                "arg1",
                "ckpt_path=/weights/rf3_foundry_01_24_latest_remapped.ckpt",
                "arg2",
            ]
        );
    }

    #[test]
    fn rf3_inserts_default_ckpt_path_at_end_when_only_one_arg() {
        // len=1 before insert → insert at min(1,2)=1 (end)
        let args = vec!["rf3".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "rf3",
                "ckpt_path=/weights/rf3_foundry_01_24_latest_remapped.ckpt",
            ]
        );
    }

    #[test]
    fn rf3_inserts_at_position_2_when_two_args() {
        // len=2 before insert → insert at min(2,2)=2 (end)
        let args = vec!["rf3".into(), "inputs=foo".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "rf3",
                "inputs=foo",
                "ckpt_path=/weights/rf3_foundry_01_24_latest_remapped.ckpt",
            ]
        );
    }

    #[test]
    fn rf3_does_not_overwrite_existing_ckpt_path() {
        let args = vec!["rf3".into(), "ckpt_path=/custom/model.ckpt".into()];
        let result = with_default_checkpoints(args.clone(), WEIGHTS);
        assert_eq!(result, args);
    }

    // ── rfd3 ────────────────────────────────────────────────────────────────

    #[test]
    fn rfd3_inserts_default_ckpt_path_at_position_1() {
        let args = vec!["rfd3".into(), "inputs=foo".into(), "out_dir=bar".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(
            result,
            vec![
                "rfd3",
                "ckpt_path=/weights/rfd3_latest.ckpt",
                "inputs=foo",
                "out_dir=bar",
            ]
        );
    }

    #[test]
    fn rfd3_only_arg_inserts_ckpt_path_at_position_1() {
        let args = vec!["rfd3".into()];
        let result = with_default_checkpoints(args, WEIGHTS);
        assert_eq!(result, vec!["rfd3", "ckpt_path=/weights/rfd3_latest.ckpt"]);
    }

    #[test]
    fn rfd3_does_not_overwrite_existing_ckpt_path() {
        let args = vec!["rfd3".into(), "ckpt_path=/custom/model.ckpt".into()];
        let result = with_default_checkpoints(args.clone(), WEIGHTS);
        assert_eq!(result, args);
    }

    // ── unknown command ──────────────────────────────────────────────────────

    #[test]
    fn unknown_command_returns_args_unchanged() {
        let args = vec!["proteinmpnn".into(), "--flag".into(), "value".into()];
        let result = with_default_checkpoints(args.clone(), WEIGHTS);
        assert_eq!(result, args);
    }

    // ── custom weights path ──────────────────────────────────────────────────

    #[test]
    fn uses_provided_weights_path() {
        let args = vec!["mpnn".into()];
        let result = with_default_checkpoints(args, "/custom/weights");
        assert_eq!(
            result,
            vec![
                "mpnn",
                "--checkpoint_path",
                "/custom/weights/ligandmpnn_v_32_010_25.pt",
            ]
        );
    }
}
