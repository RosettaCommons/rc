use crate::app::{ContainerRunSpec, RunSpec};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container = ContainerRunSpec::with_prefixed_args(
        "rosettacommons/proteinmpnn",
        ["--out_folder=/w"],
        app_args,
    )
    .working_dir("/w");

    RunSpec::new(container, None)
}
