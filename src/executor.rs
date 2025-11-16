use std::path::PathBuf;

use anyhow::{Ok, Result, anyhow};
use yansi::Paint;

use super::App;
use crate::{ContainerEngine, docker};

// struct RunCommand {
//     app: String,
//     args: Vec<String>,
//     working_dir: PathBuf,
// }

// pub struct Handler {
//     container: &'static str,
//     function: fn(container: String, args: Vec<String>) -> Result<()>,
// }

// impl App {
//     fn handler(self) -> Handler {
//         match self {
//             App::Score => Handler {
//                 container: "rosettacommons/rosetta:serial",
//                 function: foo_score,
//             },
//             App::Rosetta => Handler {
//                 container: "rosettacommons/rosetta:serial",
//                 function: docker::run_docker,
//             },
//         }
//     }

//     pub fn execute(self, args: Vec<String>) -> Result<()> {
//         let Handler {
//             container,
//             function,
//         } = self.handler();
//         function(container.to_string(), args)
//     }
// }

// fn foo_score(_container: String, _args: Vec<String>) -> Result<()> {
//     println!("Running score command");
//     todo!();
// }

pub struct Image(pub String);

impl Image {
    fn new(app: &App) -> Self {
        match app {
            App::Score => Image("rosettacommons/rosetta:serial".to_string()),
            App::Rosetta => Image("rosettacommons/rosetta:serial".to_string()),
        }
    }
}

pub fn run(
    app: &App,
    app_args: &Vec<String>,
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

    // let command = RunCommand {
    //     app: app.to_string(),
    //     args: app_args.clone(),
    //     working_dir,
    // };

    let image = Image::new(app);

    match container_engine {
        ContainerEngine::Docker => docker::run_docker(image, app_args.clone(), working_dir)?,
        _ => Err(anyhow!("Unimplemented container type: {container_engine}"))?,
    }

    // let handler = app.handler();
    // //handler.function(container, command.args);

    // app.execute(app_args.clone())
    Ok(())
}
