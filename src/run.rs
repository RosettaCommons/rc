use std::path::PathBuf;

use anyhow::Result;
use clap::ValueEnum;
use yansi::Paint;

use crate::{app, executor};

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
#[strum(serialize_all = "lowercase")] //  "kebab-case"
pub enum ContainerEngine {
    Docker,
    Singularity,
    Apptainer,
    None,
}

pub fn run(
    app: &app::App,
    app_args: Vec<String>,
    container_engine: &ContainerEngine,
    working_dir: PathBuf,
) -> Result<()> {
    println!(
        "Running app: {} in directory: {}{}",
        app.green(),
        working_dir.display(),
        if app_args.is_empty() {
            "".into()
        } else {
            format!(
                " with arguments: {}",
                format!("{:?}", app_args).bright_blue()
            )
        }
    );

    executor::Executor::new(app.to_owned(), *container_engine, working_dir)
        .execute(app.run_spec(app_args))
}
