use anyhow::Result;
use anyhow::anyhow;
use yansi::Paint;

use crate::util::ensure_dir_signature;
use crate::{ContainerEngine, app::RunSpec, executor::Executor};

impl Executor {
    pub(super) fn execute_native(&self, _spec: RunSpec) -> Result<()> {
        assert!(matches!(self.engine, ContainerEngine::None));

        Self::check_if_pixi_is_installed()?;

        let pixi_evn_root = self.working_dir.join(format!("{}.pixi", self.app));

        ensure_dir_signature(&pixi_evn_root, &["qwe", &_spec.image.0], |_d| Ok(()))?;

        //write_signature(&self.root, &hash.to_string())?;

        todo!("ContainerEngine::None")
    }

    /// Check if Pixi is installed
    fn check_if_pixi_is_installed() -> Result<()> {
        match which::which("pixi") {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!(
                "Pixi is not installed or not in PATH, please run `{}` to install Pixi or visit {} for more information",
                "curl -fsSL https://pixi.sh/install.sh | sh".green(),
                "https://pixi.sh".bright_blue().underline()
            )),
        }
    }
}
