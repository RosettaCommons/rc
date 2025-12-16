use crate::app::RunSpec;

pub fn picap(mut app_args: Vec<String>) -> RunSpec {
    app_args.splice(
        0..0,
        [
            "--out_folder=/w".into(),
            "--checkpoint_protein_mpnn".into(),
            "/app/ligandmpnn/model_params/proteinmpnn_v_48_020.pt".into(),
        ],
    );
    RunSpec::new("rosettacommons/picap", app_args).working_dir("/w")
}
