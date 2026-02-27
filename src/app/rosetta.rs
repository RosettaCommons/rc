use camino::Utf8Path;

use crate::{
    app::{ContainerRunSpec, NativeRunSpec},
    util::include_asset,
};

pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
    ContainerRunSpec::new("rosettacommons/rosetta:serial", app_args).working_dir("/w")
}

pub fn native_spec(mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
    app_args.insert(0, format!("cd {working_dir} &&"));
    NativeRunSpec::new(include_asset!("pixi/rosetta.toml"), app_args)
}
