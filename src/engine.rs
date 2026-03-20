mod docker;
mod hpc;
mod native;

use anyhow::Result;
use camino::Utf8Path;
use clap::ValueEnum;

use crate::app::AppSpec;

pub use hpc::hpc_image_path;
pub use native::pixi_evn_root;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "lowercase")] //  "kebab-case"
pub enum ContainerEngine {
    Docker,
    Singularity,
    Apptainer,
    None,
}

pub trait Engine {
    fn execute(&self, app: &dyn AppSpec, args: Vec<String>, working_dir: &Utf8Path) -> Result<()>;
    fn install(&self, app: &dyn AppSpec) -> Result<()>;
    fn clean(&self, app: &dyn AppSpec) -> Result<()>;
}

impl ContainerEngine {
    pub fn engine(self) -> &'static dyn Engine {
        match self {
            ContainerEngine::Docker => &docker::DOCKER,
            ContainerEngine::Singularity => &hpc::SINGULARITY,
            ContainerEngine::Apptainer => &hpc::APPTAINER,
            ContainerEngine::None => &native::NATIVE,
        }
    }
}
