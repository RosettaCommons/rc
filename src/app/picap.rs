use crate::app::ContainerRunSpec;

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::with_prefixed_args(
        "rosettacommons/picap",
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
    // RunSpec::new("rosettacommons/picap", app_args).working_dir("/w")
}
