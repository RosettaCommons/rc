use std::fs;

use anyhow::Result;
use anyhow::anyhow;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use yansi::Paint;

use crate::app::AppSpec;
use crate::engine::Engine;
use crate::telemetry::Telemetry;
use crate::util::Command;
use crate::util::dirs::cache_root;
use crate::util::ensure_dir_signature;

pub struct NativeEngine;
pub static NATIVE: NativeEngine = NativeEngine;

impl Engine for NativeEngine {
    fn execute(&self, app: &dyn AppSpec, args: Vec<String>, work_dir: &Utf8Path) -> Result<()> {
        let spec = app.native_spec(args, work_dir);

        let pixi_evn_root = build_pixi_env(app)?;

        let new_args = spec
            .args
            // .into_iter()
            // .map(|arg| shell_escape::escape(arg.into()).to_string())
            // .collect::<Vec<_>>()
            .join(" ");

        let command = Command::new("pixi")
            // .cd(&pixi_evn_root)
            .arg("run")
            .args(["--manifest-path", pixi_evn_root.join("pixi.toml").as_str()])
            .arg("execute")
            .arg(new_args)
            .live();

        let result = command.call();

        let t = Telemetry::new(work_dir);

        let logs = format!(
            "{command}\nprocess success: {}\n{}\n{}\n{}\n",
            result.success, result.stdout, result.stderr, result.stderr
        );

        fs::write(t.log_file_name(), logs)?;

        if !result.success {
            eprintln!(
                "{}",
                format!("Native run for {} exited with non-zero status", app.name())
                    .bright_red()
                    .bold()
            );
            return Err(anyhow::anyhow!(
                "Native run for {} exited with non-zero status",
                app.name()
            ));
        }

        println!(
            "{}",
            format!(
                "The exact command line used and full log saved into {:?}\nScratch dir for this run is: {:?}\n",
                t.log_file_name(), t.scratch_dir()
            )
            .blue()
            .dim()
        );

        Ok(())
    }

    fn install(&self, app: &dyn AppSpec) -> Result<()> {
        build_pixi_env(app)?;
        Ok(())
    }

    fn clean(&self, app: &dyn AppSpec) -> Result<()> {
        let pixi_evn_root = pixi_evn_root(app);

        if pixi_evn_root.exists() {
            fs::remove_dir_all(&pixi_evn_root)?;
        }

        Ok(())
    }
}

fn build_pixi_env(app: &dyn AppSpec) -> Result<Utf8PathBuf, anyhow::Error> {
    let pixi_recipe = app
        .pixi_recipe()
        .unwrap_or_else(|| unimplemented!("Native run for {} is not supported", app.name().red()));

    check_if_pixi_is_installed()?;

    let pixi_evn_root = pixi_evn_root(app);

    ensure_dir_signature(&pixi_evn_root, &[app.name(), pixi_recipe], |d| {
        std::fs::write(d.join("pixi.toml"), pixi_recipe)?;
        Command::new("pixi")
            .cd(d)
            .arg("run")
            .arg("setup")
            .live()
            .exec()?;
        Ok(())
    })?;
    Ok(pixi_evn_root)
}

pub fn pixi_evn_root(app: &dyn AppSpec) -> Utf8PathBuf {
    cache_root().join(format!("native/{}", app.name()))
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
