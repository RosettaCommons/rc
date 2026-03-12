use camino::Utf8Path;

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec},
    util::include_asset,
};

pub struct Rosetta;
pub static ROSETTA: Rosetta = Rosetta;

impl AppSpec for Rosetta {
    fn container_image(&self) -> &'static str {
        "rosettacommons/rosetta:serial"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/rosetta.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::new(app_args).working_dir("/w")
    }

    fn native_spec(&self, mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        app_args.insert(0, format!("cd {working_dir} &&"));
        NativeRunSpec::new(app_args)
    }
}
