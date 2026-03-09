use camino::Utf8Path;

use crate::spec::{AppSpec, ContainerConfig, NativeRunSpec};

pub struct Pyrosetta;
pub static PYROSETTA: Pyrosetta = Pyrosetta;

impl AppSpec for Pyrosetta {
    fn container_image(&self) -> &'static str {
        "rosettacommons/rosetta:serial"
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(["python"], app_args).working_dir("/w")
    }
    fn native_spec(&self, _app_args: Vec<String>, _working_dir: &Utf8Path) -> NativeRunSpec {
        unimplemented!()
    }
}
