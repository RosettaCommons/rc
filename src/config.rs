use serde::Serialize;
use strum::IntoEnumIterator;
use yansi::Paint;

use crate::{
    app::App,
    engine::{hpc_image_path, pixi_evn_root},
};

#[derive(Serialize)]
struct Config {
    apps: Vec<AppInfo>,
}

#[derive(Serialize)]
struct AppInfo {
    name: String,
    container_image: String,
    hpc_image_path: String,
    native_root: Option<String>,
}

pub fn config_show(json: bool) -> std::result::Result<(), anyhow::Error> {
    let config = Config {
        apps: App::iter()
            .map(App::spec)
            .map(|a| AppInfo {
                name: a.name().to_lowercase(),
                container_image: a.container_image().into(),
                hpc_image_path: hpc_image_path(a.container_image()).into(),
                native_root: a.pixi_recipe().map(|_| pixi_evn_root(a).into()),
            })
            .collect(),
    };

    if json {
        let output = serde_json::to_string_pretty(&config).unwrap();
        println!("{}", output);
    } else {
        println!("{}", "Apps:".bold());
        for app in &config.apps {
            println!(
                "  {} {}",
                "•".bold(),
                app.name.to_lowercase().green().bold()
            );
            println!(
                "    {:<20} {}",
                "Container Image:".dim(),
                app.container_image
            );
            println!("    {:<20} {}", "HPC Image Path:".dim(), app.hpc_image_path);
            println!(
                "    {:<20} {}",
                "Native Root:".dim(),
                app.native_root.as_deref().unwrap_or("—")
            );
            println!();
        }
    }

    Ok(())
}
