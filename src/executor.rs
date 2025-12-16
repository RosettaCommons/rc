mod docker;
mod hpc_container;

use std::path::{Path, PathBuf};

use anyhow::Result;
use yansi::Paint;

use super::App;
use crate::{
    ContainerEngine,
    app::{Image, RunSpec},
};

pub struct Executor {
    app: App,
    working_dir: PathBuf,
    engine: ContainerEngine,
}

impl Executor {
    fn new(app: App, engine: ContainerEngine, working_dir: PathBuf) -> Self {
        Executor {
            app,
            working_dir,
            engine,
        }
    }

    pub fn execute(&self, spec: RunSpec) -> Result<()> {
        match self.engine {
            ContainerEngine::Docker => self.execute_with_docker(spec),

            ContainerEngine::Singularity | ContainerEngine::Apptainer => {
                self.execute_with_hpc_container_engine(spec)
            }

            ContainerEngine::None => todo!("ContainerEngine::None"),
        }
    }

    fn log_execute_info(&self, spec: &RunSpec) {
        println!(
            "Running {} container: {} working directory: {:?}",
            self.engine, spec.image.0, self.working_dir
        );
        if !spec.args.is_empty() {
            println!("With arguments: {:?}", spec.args);
        }
    }
}

pub fn run(
    app: &App,
    app_args: Vec<String>,
    container_engine: &ContainerEngine,
    working_dir: PathBuf,
) -> Result<()> {
    println!(
        "Running app: {} in directory: {}",
        app.green(),
        working_dir.display()
    );
    if !app_args.is_empty() {
        println!(
            "With arguments: {}",
            format!("{:?}", app_args).bright_blue()
        );
    }

    Executor::new(app.to_owned(), *container_engine, working_dir).execute(app.run_spec(app_args))
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
