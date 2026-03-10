use std::fs;

use anyhow::Result;
use camino::Utf8PathBuf;
use strum::IntoEnumIterator;
use yansi::Paint;

use crate::{
    app::{App, AppSpec},
    engine::ContainerEngine,
    util::dirs::cache_root,
};

// pub struct Executor {
//     app: App,
//     working_dir: Utf8PathBuf,
//     engine: ContainerEngine,
// }

// impl Executor {
//     pub fn new(app: App, engine: ContainerEngine, working_dir: Utf8PathBuf) -> Self {
//         Executor {
//             app,
//             working_dir,
//             engine,
//         }
//     }

//     pub fn execute(&self, app_args: Vec<String>) -> Result<()> {
//         match self.engine {
//             ContainerEngine::Docker => {
//                 self.execute_with_docker(self.app.spec().container_spec(app_args))
//             }

//             ContainerEngine::Singularity | ContainerEngine::Apptainer => {
//                 self.execute_with_hpc_container_engine(self.app.spec().container_spec(app_args))
//             }

//             ContainerEngine::None => {
//                 self.execute_native(self.app.spec().native_spec(app_args, &self.working_dir))
//             }
//         }
//     }
// }

pub fn run(
    app: &dyn AppSpec,
    args: Vec<String>,
    engine: ContainerEngine,
    working_dir: Utf8PathBuf,
) -> Result<()> {
    println!(
        "Running app: {} in directory: {}{}",
        app.name().green(),
        working_dir,
        if args.is_empty() {
            "".into()
        } else {
            format!(" with arguments: {}", format!("{:?}", args).bright_blue())
        }
    );

    engine.engine().execute(app, args, &working_dir)

    //driver::Executor::new(app.to_owned(), *container_engine, working_dir).execute(app_args)
}

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

pub fn clean_all() -> Result<()> {
    let cache_root = cache_root();
    println!("Cleaning up cache dir {:?}...", cache_root.bright().red());
    fs::remove_dir_all(cache_root)?;
    Ok(())
}
