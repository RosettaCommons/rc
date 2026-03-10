use camino::Utf8Path;

use crate::{
    app::rosetta,
    app::{AppSpec, ContainerConfig, NativeRunSpec},
};

pub struct Score;
pub static SCORE: Score = Score;

impl AppSpec for Score {
    fn container_image(&self) -> &'static str {
        rosetta::ROSETTA.container_image()
    }

    /// Pixi TOML recipe for native execution.
    /// `None` (default) means this app does not support native execution.
    fn pixi_recipe(&self) -> Option<&'static str> {
        rosetta::ROSETTA.pixi_recipe()
    }

    fn container_spec(&self, mut args: Vec<String>) -> ContainerConfig {
        args.insert(0, "score".into());
        rosetta::ROSETTA.container_spec(args)
    }

    fn native_spec(&self, mut args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        args.insert(0, "score".into());
        rosetta::ROSETTA.native_spec(args, working_dir)
    }
}
// pub fn container_spec(app_args: Vec<String>) -> ContainerRunSpec {
//     ContainerRunSpec::with_prefixed_args("rosettacommons/rosetta:serial", ["score"], app_args)
//         .working_dir("/w")
// }

// pub fn native_spec(mut app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
//     app_args.insert(0, "score".into());
//     super::rosetta::native_spec(app_args, working_dir)
// }
