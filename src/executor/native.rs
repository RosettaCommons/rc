use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use home::home_dir;
use yansi::Paint;

use crate::run;
use crate::util::Command;
use crate::util::ensure_dir_signature;
use crate::{app::RunSpec, executor::Executor};

impl Executor {
    pub(super) fn execute_native(&self, spec: RunSpec) -> Result<()> {
        assert!(matches!(self.engine, run::ContainerEngine::None));

        let recipe = spec
            .native
            .with_context(|| format!("Pixi recipe for app '{}' was not found", self.app))?
            .pixi;

        Self::check_if_pixi_is_installed()?;

        //let pixi_evn_root = self.working_dir.join(format!("{}.pixi", self.app));
        let pixi_evn_root = home_dir()
            .unwrap()
            .join(format!(".cache/rosettacommons/rc/native/{}.pixi", self.app));

        ensure_dir_signature(
            &pixi_evn_root,
            &[spec.container.image.0.as_ref(), recipe.as_ref()],
            |d| {
                std::fs::write(d.join("pixi.toml"), recipe.as_ref())?;
                Command::new("pixi")
                    .cd(d)
                    .arg("run")
                    .arg("setup")
                    .live()
                    .exec()?;
                Ok(())
            },
        )?;

        Ok(())
    }

    /// Check if Pixi is installed, fail if not
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
