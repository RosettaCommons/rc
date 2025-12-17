use std::fs;

use anyhow::Result;
use yansi::Paint;

use crate::{
    ContainerEngine,
    app::{MountRole, RunSpec},
    executor::{Executor, Telemetry},
    util,
};

impl Executor {
    pub(super) fn execute_with_docker(&self, spec: RunSpec) -> Result<()> {
        assert!(matches!(self.engine, ContainerEngine::Docker));

        self.log_execute_info(&spec);

        let mut options = format!("--volume {}:/w --workdir /w", self.working_dir.display());

        #[cfg(unix)]
        {
            let uid = users::get_current_uid();
            let gid = users::get_current_gid();
            options.push_str(&format!(" --user {uid}:{gid}"));
        }

        let t = Telemetry::new(&self.working_dir);

        if let Some(scratch) = &spec.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            options.push_str(&format!(
                " --volume {}:/{scratch}",
                d.to_str().expect("path is not valid UTF-8")
            ));
            fs::create_dir_all(&d)?;
        }

        let mut command = util::Command::new("docker");

        command
            .arg("run")
            .args(options.split(' '))
            .arg(&spec.image.0)
            .args(spec.args.clone())
            .message(format!(
                "Executing {} with arguments: {:?}",
                self.app, spec.args
            ));

        println!("Running {command}");

        let result = command.live().call();

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
