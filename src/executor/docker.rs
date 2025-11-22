use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use yansi::Paint;

use crate::{executor::Image, util};

pub fn run_docker(Image(image): Image, args: Vec<String>, working_dir: PathBuf) -> Result<()> {
    println!("Running docker container: {image} working directory: {working_dir:?}");
    if !args.is_empty() {
        println!("With arguments: {:?}", args);
    }

    let mut options = format!("--volume {}:/w --workdir /w", working_dir.display());

    #[cfg(unix)]
    {
        let uid = users::get_current_uid();
        let gid = users::get_current_gid();
        options.push_str(&format!(" --user {uid}:{gid}"));
    }

    let command_line = format!("docker run {options} {image} {}", args.join(" "));

    println!("Running {command_line}");

    // let status = std::process::Command::new("sh")
    //     .arg("-c")
    //     .arg(command_line)
    //     .status()?;

    let results = util::Command::shell(&command_line).try_call();

    println!("{}", results.stdout.bright_black());
    eprintln!("{}", results.stderr.bright_red());

    let logs = format!(
        "{command_line}\nprocess success: {}\n{}\n{}\n{}\n",
        results.success, results.stdout, results.stderr, results.stderr
    );

    let t = Telemetry::new(&working_dir);

    fs::write(t.log_file_name(), logs)?;

    if !results.success {
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

// fn get_available_log_file_name(working_dir: &Path) -> PathBuf {
//     let mut i: u32 = 0;
//     loop {
//         let log_file = working_dir.join(format!(".{i:04}.rc.log"));
//         if !log_file.exists() {
//             return log_file;
//         }
//         i += 1;
//     }
// }
