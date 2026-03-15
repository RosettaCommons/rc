use anyhow::Result;
use camino::Utf8PathBuf;
use strum::IntoEnumIterator;
use yansi::Paint;

use crate::{
    app::{App, AppSpec},
    engine::ContainerEngine,
    util::yansi::PaintExt,
};

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
}

pub fn install(app: App, engine: ContainerEngine) -> Result<()> {
    println!("Install app: {}", app.bright_green());

    engine.engine().install(app.spec())
}

pub fn clean(app: Option<App>, container_engine: Option<ContainerEngine>) -> Result<()> {
    let apps = match app {
        Some(app) => vec![app.spec()],
        None => App::iter().map(App::spec).collect(),
    };

    let engines: Vec<ContainerEngine> = if let Some(engine) = container_engine {
        vec![engine]
    } else {
        ContainerEngine::iter()
            // when only need a single HPC engine
            .filter(|a| a != &ContainerEngine::Apptainer)
            .collect()
    };

    for app in apps {
        for engine in &engines {
            println!(
                "{}",
                format!("Cleaning {engine} engine data for {:?} app...", app.name()).orange()
            );

            engine.engine().clean(app)?;
            // println!();
        }
    }

    Ok(())
}

// pub fn clean_all() -> Result<()> {
//     let cache_root = cache_root();
//     println!(
//         "Cleaning up cache dir {:?}...",
//         cache_root.bright().orange()
//     );
//     fs::remove_dir_all(cache_root)?;
//     Ok(())
// }
