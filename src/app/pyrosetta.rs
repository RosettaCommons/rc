use crate::app::ContainerRunSpec;

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::with_prefixed_args("rosettacommons/rosetta:serial", ["python"], app_args)
        .working_dir("/w")
}
