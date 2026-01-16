use crate::app::{ContainerRunSpec, RunSpec};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container = ContainerRunSpec::new(
        "rosettacommons/ligandmpnn",
        [
            "--out_folder=/w",
            "--checkpoint_protein_mpnn",
            "/app/ligandmpnn/model_params/proteinmpnn_v_48_020.pt",
        ]
        .into_iter()
        .map(Into::into)
        .chain(app_args)
        .collect(),
    )
    .working_dir("/w");

    RunSpec::new(container, None)

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
