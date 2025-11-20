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
    fs::write(get_available_log_file_name(&working_dir), logs)?;

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

    Ok(())
}

fn get_available_log_file_name(working_dir: &Path) -> PathBuf {
    let mut i: u32 = 0;
    loop {
        let log_file = working_dir.join(format!(".{i:04}.rc.log"));
        if !log_file.exists() {
            return log_file;
        }
        i += 1;
    }
}
