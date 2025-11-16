use std::path::PathBuf;

use anyhow::Result;
use yansi::Paint;

use crate::executor::Image;

pub fn run_docker(Image(image): Image, args: Vec<String>, working_dir: PathBuf) -> Result<()> {
    println!("Running docker container: {image} working directory: {working_dir:?}");
    if !args.is_empty() {
        println!("With arguments: {:?}", args);
    }

    let uid = users::get_current_uid();
    let gid = users::get_current_gid();

    let options = format!(
        "--volume {}:/w --user {uid}:{gid} --workdir /w",
        working_dir.display()
    );

    let command_line = format!("docker run {options} {image} {}", args.join(" "));

    println!("Running {command_line}");

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command_line)
        .status()?;

    if !status.success() {
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
