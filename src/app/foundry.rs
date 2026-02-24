use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{ContainerRunSpec, NativeRunSpec},
    util::include_asset,
};

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

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
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

fn make_absolute(working_dir: &Utf8Path, path_str: &str) -> Utf8PathBuf {
    let path = Utf8PathBuf::from(path_str);
    if path.is_absolute() {
        path
    } else {
        working_dir.join(path)
    }
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
    const OPTIONS: [&str; 2] = ["--structure_path", "--out_directory"];
    const OUTPUT_OPTION: &str = OPTIONS[1];

    let mut output_option_present = false;

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

pub fn native_spec(app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
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

    NativeRunSpec::new(include_asset!("pixi/foundry.toml"), app_args)
}
