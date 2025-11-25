use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use yansi::Paint;

use crate::{executor::Executor, util};

impl Executor {
    pub fn execute_with_docker(&self) -> Result<()> {
        println!(
            "Running docker container: {} working directory: {:?}",
            self.image.0, self.working_dir
        );
        if !self.args.is_empty() {
            println!("With arguments: {:?}", self.args);
        }

        let mut options = format!("--volume {}:/w --workdir /w", self.working_dir.display());

        #[cfg(unix)]
        {
            let uid = users::get_current_uid();
            let gid = users::get_current_gid();
            options.push_str(&format!(" --user {uid}:{gid}"));
        }

        let t = Telemetry::new(&self.working_dir);

        if let Some(scratch) = &self.scratch {
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
            .arg(&self.image.0)
            .args(self.args.clone());

        println!("Running {command}");

        let result = command.call();

        // let command_line = format!(
        //     "docker run {options} {} {}",
        //     self.image.0,
        //     self.args.join(" ")
        // );
        // println!("Running {command_line}");
        // let result = util::Command::shell(&command_line).try_call();

        println!("{}", result.stdout.bright_black());
        eprintln!("{}", result.stderr.bright_red());

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

struct Telemetry {
    working_dir: PathBuf,
    prefix: String,
}

impl Telemetry {
    fn new(working_dir: &Path) -> Self {
        let mut i: u32 = 0;
        loop {
            let prefix = format!(".{i:04}.rc");
            i += 1;

            let r = Telemetry {
                working_dir: working_dir.to_path_buf(),
                prefix: prefix.to_string(),
            };

            if r.log_file_name().exists() || r.scratch_dir().exists() {
                continue;
            }

            break r;
        }
    }

    pub fn log_file_name(&self) -> PathBuf {
        self.working_dir.join(format!("{}.log", self.prefix))
    }

    pub fn scratch_dir(&self) -> PathBuf {
        self.working_dir.join(format!("rc.scratch/{}", self.prefix))
    }
}
