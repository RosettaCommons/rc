use camino::Utf8Path;

use crate::app::{AppSpec, ContainerConfig, NativeRunSpec};

pub struct Patchman;
pub static PATCHMAN: Patchman = Patchman;

impl AppSpec for Patchman {
    fn container_image(&self) -> &'static str {
        "rosettacommons/patchman"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        None
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        assert!(
            (app_args.len() >= 2),
            "PatchMAN app requires that the last two arguments are a pdb-file and sequence"
        );

        // if let Some(arg) = app_args.iter_mut().rev().nth(1) {
        //     *arg = make_absolute("/w".into(), arg).into();
        // }

        ContainerConfig::with_prefixed_args(["PatchMAN_protocol_dask.py", "-w", "/w"], app_args)
            .working_dir("/w")
    }

    fn native_spec(&self, mut _app_args: Vec<String>, _working_dir: &Utf8Path) -> NativeRunSpec {
        unimplemented!("Patchman does not support native execution")
    }
}
