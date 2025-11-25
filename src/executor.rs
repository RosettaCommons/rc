mod docker;
mod singularity;

use std::path::PathBuf;

use anyhow::Result;
use yansi::Paint;

use super::App;
use crate::ContainerEngine;

pub struct Image(String);

// impl Image {
//     fn new(app: &App) -> Self {
//         match app {
//             App::Score => Image("rosettacommons/rosetta:serial".to_string()),
//             App::Rosetta => Image("rosettacommons/rosetta:serial".to_string()),
//             App::Rfdiffusion => Image("rosettacommons/rfdiffusion".to_string()),
//         }
//     }
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
    pub fn new(
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
            ContainerEngine::Singularity => self.execute_with_singularity(),
            ContainerEngine::Apptainer => todo!(),
            ContainerEngine::None => todo!(),
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
