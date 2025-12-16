use crate::app::RunSpec;

pub fn pyrosetta(mut app_args: Vec<String>) -> RunSpec {
    app_args.insert(0, "python".into());
    RunSpec::new("rosettacommons/rosetta:serial", app_args).working_dir("/w")
}
