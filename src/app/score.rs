use camino::Utf8Path;

use crate::app::{ContainerRunSpec, NativeRunSpec};

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::with_prefixed_args("rosettacommons/rosetta:serial", ["score"], app_args)
        .working_dir("/w")
}

pub fn native_spec(mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
    app_args.insert(0, "score".into());
    super::rosetta::native_spec(app_args, working_dir)
}
