use crate::app::ContainerRunSpec;

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::new("rosettacommons/rosetta:serial", app_args).working_dir("/w")
}
