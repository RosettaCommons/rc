use std::fs;

use anyhow::Result;
use camino::Utf8Path;
use yansi::Paint;

use crate::{
    app::{AppSpec, MountRole},
    engine::Engine,
    telemetry::Telemetry,
    util,
};

pub struct DockerEngine;
pub static DOCKER: DockerEngine = DockerEngine;

impl Engine for DockerEngine {
    fn execute(&self, app: &dyn AppSpec, args: Vec<String>, working_dir: &Utf8Path) -> Result<()> {
        let spec = app.container_spec(args);

        util::Command::shell(format!(
            "docker image inspect {0} >/dev/null 2>&1 || docker image pull {0}",
            app.container_image()
        ))
        .live()
        .exec()?;

        //self.log_execute_info(&spec);

        let mut options = format!("--volume {}:/w --workdir /w", working_dir);

        #[cfg(unix)]
        {
            let uid = nix::unistd::getuid().as_raw();
            let gid = nix::unistd::getgid().as_raw();
            options.push_str(&format!(" --user {uid}:{gid}"));
        }

        let t = Telemetry::new(working_dir);

        if let Some(scratch) = &spec.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            options.push_str(&format!(" --volume {d}:/{scratch}"));
            fs::create_dir_all(&d)?;
        }

        if let Some(entrypoint) = &spec.entrypoint {
            options.push_str(&format!(" --entrypoint {entrypoint}"));
        }

        let command = util::Command::new("docker")
            .arg("run")
            .args(options.split(' '))
            .arg(app.container_image())
            .args(spec.args.clone())
            // .message(format!(
            //     "Executing {} with arguments: {:?}",
            //     self.app, spec.args
            // ))
            .live();

        //let result = command.live().call();
        let result = command.call();

        // let command_line = format!(
        //     "docker run {options} {} {}",
        //     self.image.0,
        //     self.args.join(" ")
        // );
        // println!("Running {command_line}");
        // let result = util::Command::shell(&command_line).try_call();

        //println!("{}", result.stdout.bright_black());
        //eprintln!("{}", result.stderr.bright_red());

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
}
