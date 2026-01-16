use crate::app::{ContainerRunSpec, RunSpec};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container = ContainerRunSpec::new(
        "rosettacommons/rosetta:serial",
        ["score"]
            .into_iter()
            .map(Into::into)
            .chain(app_args)
            .collect(),
    )
    .working_dir("/w");

    RunSpec::new(container, None)
}
