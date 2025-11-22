mod docker;

use std::path::PathBuf;

use anyhow::{Ok, Result, anyhow};
use yansi::Paint;

use super::App;
use crate::ContainerEngine;

pub struct Image(String);

impl Image {
    fn new(app: &App) -> Self {
        match app {
            App::Score => Image("rosettacommons/rosetta:serial".to_string()),
            App::Rosetta => Image("rosettacommons/rosetta:serial".to_string()),
            App::Rfdiffusion => Image("rosettacommons/rfdiffusion".to_string()),
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

    let image = Image::new(app);

    let mut app_args = app_args.clone();

    if matches!(app, App::Score) {
        app_args.insert(0, "score".into());
    }

    match container_engine {
        ContainerEngine::Docker => docker::run_docker(image, app_args, working_dir)?,
        _ => Err(anyhow!("Unimplemented container type: {container_engine}"))?,
    }

    Ok(())
}
