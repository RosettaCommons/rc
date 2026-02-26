use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{ContainerRunSpec, NativeRunSpec},
    util::include_asset,
};

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::with_prefixed_args(
        "rosettacommons/ligandmpnn",
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

pub fn native_spec(app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
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

    NativeRunSpec::new(include_asset!("pixi/ligandmpnn.toml"), app_args)
}
