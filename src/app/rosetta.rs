use crate::app::RunSpec;

pub fn rosetta(app_args: Vec<String>) -> RunSpec {
    RunSpec::new("rosettacommons/rosetta:serial", app_args).working_dir("/w")
}
