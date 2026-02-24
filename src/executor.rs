mod docker;
mod hpc_container;
mod native;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{App, Image},
    run::ContainerEngine,
};

pub struct Executor {
    app: App,
    working_dir: Utf8PathBuf,
    engine: ContainerEngine,
}

impl Executor {
    pub fn new(app: App, engine: ContainerEngine, working_dir: Utf8PathBuf) -> Self {
        Executor {
            app,
            working_dir,
            engine,
        }
    }

    pub fn execute(&self, app_args: Vec<String>) -> Result<()> {
        match self.engine {
            ContainerEngine::Docker => self.execute_with_docker(self.app.container_spec(app_args)),

            ContainerEngine::Singularity | ContainerEngine::Apptainer => {
                self.execute_with_hpc_container_engine(self.app.container_spec(app_args))
            }

            ContainerEngine::None => {
                self.execute_native(self.app.native_spec(app_args, &self.working_dir))
            }
        }
    }

    // fn log_execute_info(&self, spec: &RunSpec) {
    //     println!(
    //         "Running {} container: {} working directory: {:?}",
    //         self.engine, spec.image.0, self.working_dir
    //     );
    //     if !spec.args.is_empty() {
    //         println!("With arguments: {:?}", spec.args);
    //     }
    // }
}

struct Telemetry {
    working_dir: Utf8PathBuf,
    prefix: String,
}

impl Telemetry {
    fn new(working_dir: &Utf8Path) -> Self {
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

    pub fn log_file_name(&self) -> Utf8PathBuf {
        self.working_dir.join(format!("{}.log", self.prefix))
    }

    pub fn scratch_dir(&self) -> Utf8PathBuf {
        self.working_dir.join(format!("rc.scratch/{}", self.prefix))
    }
}
