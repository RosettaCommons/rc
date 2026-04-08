use camino::Utf8Path;

use crate::app::{AppSpec, ContainerConfig, NativeRunSpec};

pub struct Colabfold;
pub static COLABFOLD: Colabfold = Colabfold;

impl AppSpec for Colabfold {
    fn container_image(&self) -> &'static str {
        "ghcr.io/sokrypton/colabfold:1.6.0-cuda12"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        None
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(["colabfold_batch"], app_args).working_dir("/w")
    }

    fn native_spec(&self, mut _app_args: Vec<String>, _working_dir: &Utf8Path) -> NativeRunSpec {
        unimplemented!("Colabfold does not support native execution")
    }
}
