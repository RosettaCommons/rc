use std::fs;

use crate::{app::App, run::ContainerEngine};

use anyhow::Result;
use strum::IntoEnumIterator;
use yansi::Paint;

pub fn clean(app: Option<App>, container_engine: Option<ContainerEngine>) -> Result<()> {
    let apps: Vec<App> = if app.is_some() {
        vec![app.unwrap()]
    } else {
        App::iter().collect()
    };

    let engines: Vec<ContainerEngine> = if container_engine.is_some() {
        vec![container_engine.unwrap()]
    } else {
        ContainerEngine::iter().collect()
    };

    for app in apps {
        for engine in &engines {
            match engine {
                ContainerEngine::Docker => {
                    println!("Cleaning Docker for {:?}", app.green());
                }
                ContainerEngine::Singularity | ContainerEngine::Apptainer => {
                    println!("Cleaning Singularity/Apptainer for {:?}", app.green());
                    // let spec = app.container_spec(vec![]);
                    // let image_path = Executor::image_path(&spec.image);
                    // fs::remove_file(&image_path)?;
                }

                ContainerEngine::None => {
                    println!("Cleaning Native install for {:?}", app.green());
                }
            }
        }
    }

    Ok(())
}
