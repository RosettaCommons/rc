mod docker;
mod hpc_container;

use std::path::{Path, PathBuf};

use anyhow::Result;
use yansi::Paint;

use super::App;
use crate::ContainerEngine;

struct Image(String);

// impl Image {
//     fn new(app: &App) -> Self {
//         match app {
//             App::Score => Image("rosettacommons/rosetta:serial".to_string()),
//             App::Rosetta => Image("rosettacommons/rosetta:serial".to_string()),
//             App::Rfdiffusion => Image("rosettacommons/rfdiffusion".to_string()),
//         }
//     }
// }
// #[derive(Debug, Display)]
// #[strum(serialize_all = "kebab-case")]
// enum HpcContainerRuntime {
//     Apptainer,
//     Singularity,
// }

pub struct Executor {
    app: App,
    image: Image,
    args: Vec<String>,
    working_dir: PathBuf,
    scratch: Option<String>,
    engine: ContainerEngine,
}

impl Executor {
    fn new(
        app: App,
        engine: ContainerEngine,
        image: Image,
        args: Vec<String>,
        working_dir: PathBuf,
        scarch: Option<String>,
    ) -> Self {
        Executor {
            app,
            image,
            args,
            working_dir,
            engine,
            scratch: scarch,
        }
    }

    pub fn execute(&self) -> Result<()> {
        match self.engine {
            ContainerEngine::Docker => self.execute_with_docker(),

            ContainerEngine::Singularity | ContainerEngine::Apptainer => {
                self.execute_with_hpc_container_engine()
            }

            // engine @ (ContainerEngine::Singularity | ContainerEngine::Apptainer) => {
            //     self.execute_with_hpc_container_engine(&HpcContainerEngine(engine.to_string()))
            // }
            ContainerEngine::None => todo!("ContainerEngine::None"),
        }
    }

    fn log_execute_info(&self) {
        println!(
            "Running {} container: {} working directory: {:?}",
            self.engine, self.image.0, self.working_dir
        );
        if !self.args.is_empty() {
            println!("With arguments: {:?}", self.args);
        }
    }
}

pub fn run(
    app: &App,
    mut app_args: Vec<String>,
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

    let (image, app_args, scratch) = match app {
        App::Score => (
            Image("rosettacommons/rosetta:serial".into()),
            {
                app_args.insert(0, "score".into());
                app_args
            },
            None,
        ),

        App::Rosetta => (
            Image("rosettacommons/rosetta:serial".into()),
            app_args,
            None,
        ),

        App::PyRosetta => (
            Image("rosettacommons/rosetta:serial".into()),
            {
                app_args.insert(0, "python".into());
                app_args
            },
            None,
        ),

        App::Rfdiffusion => (
            Image("rosettacommons/rfdiffusion".into()),
            {
                //app_args.insert(0, "inference.output_prefix=/w".into()); // /motifscaffolding
                app_args.extend([
                    "inference.output_prefix=/w/".into(),
                    "inference.model_directory_path=/app/RFdiffusion/models".into(),
                ]);
                app_args
            },
            Some("/app/RFdiffusion/schedules".into()),
        ),
        //_ => todo!(),
    };

    Executor::new(
        app.to_owned(),
        *container_engine,
        image,
        app_args.clone(),
        working_dir,
        scratch,
    )
    .execute()

    // match container_engine {
    //     ContainerEngine::Docker => docker::run_docker(image, app_args, working_dir)?,
    //     _ => Err(anyhow!("Unimplemented container type: {container_engine}"))?,
    // }
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
