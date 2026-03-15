use std::fs;

use anyhow::Result;
use camino::Utf8Path;
use yansi::Paint;

use crate::{
    app::{AppSpec, MountRole},
    engine::Engine,
    telemetry::Telemetry,
    util::{self},
};

pub struct DockerEngine;
pub static DOCKER: DockerEngine = DockerEngine;

impl Engine for DockerEngine {
    fn execute(&self, app: &dyn AppSpec, args: Vec<String>, working_dir: &Utf8Path) -> Result<()> {
        let spec = app.container_spec(args);

        self.install(app)?;

        let t = Telemetry::new(working_dir);

        let mut cmd = util::Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--volume")
            .arg(format!("{working_dir}:/w"))
            .arg("--workdir")
            .arg("/w");

        #[cfg(unix)]
        {
            let uid = nix::unistd::getuid().as_raw();
            let gid = nix::unistd::getgid().as_raw();
            cmd = cmd.arg("--user").arg(format!("{uid}:{gid}"));
        }

        if let Some(scratch) = spec.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            fs::create_dir_all(&d)?;
            cmd = cmd.arg("--volume").arg(format!("{d}:/{scratch}"));
        }

        if let Some(entrypoint) = &spec.entrypoint {
            cmd = cmd.arg("--entrypoint").arg(entrypoint.as_str());
        }

        let command = cmd.arg(app.container_image()).args(spec.args).live();

        let result = command.call();

        let logs = format!(
            "{command}\nprocess success: {}\n{}\n{}\n{}\n",
            result.success, result.stdout, result.stderr, result.stderr
        );

        fs::write(t.log_file_name(), logs)?;

        if !result.success {
            eprintln!(
                "{}",
                "Docker container exited with non-zero status"
                    .bright_red()
                    .bold()
            );
            return Err(anyhow::anyhow!(
                "Docker container exited with non-zero status"
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
        util::Command::shell(format!(
            "docker image inspect {0} >/dev/null 2>&1 || docker image pull {0}",
            app.container_image()
        ))
        .live()
        .exec()?;
        Ok(())
    }

    fn clean(&self, app: &dyn AppSpec) -> Result<()> {
        util::Command::new("docker")
            .arg("image")
            .arg("rm")
            .arg("-f")
            .arg(app.container_image())
            .live()
            .exec()?;
        Ok(())
    }
}
