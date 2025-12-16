use crate::app::RunSpec;

pub fn proteinmpnn(mut app_args: Vec<String>) -> RunSpec {
    app_args.splice(0..0, ["--out_folder=/w".into()]);
    RunSpec::new("rosettacommons/proteinmpnn", app_args).working_dir("/w")
}
