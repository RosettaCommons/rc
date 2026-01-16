use crate::app::{ContainerRunSpec, RunSpec};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container =
        ContainerRunSpec::new("rosettacommons/rosetta:serial", app_args).working_dir("/w");

    RunSpec::new(container, None)
}
