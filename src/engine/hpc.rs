use std::fs;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    app::{AppSpec, MountRole},
    engine::Engine,
    telemetry::Telemetry,
    util::{self, Command, dirs},
};

use anyhow::Result;
use yansi::Paint;

pub struct HpcEngine(pub &'static str);

pub static SINGULARITY: HpcEngine = HpcEngine("singularity");
pub static APPTAINER: HpcEngine = HpcEngine("apptainer");

impl Engine for HpcEngine {
    fn execute(&self, app: &dyn AppSpec, args: Vec<String>, working_dir: &Utf8Path) -> Result<()> {
        assert!(matches!(self.0, "singularity" | "apptainer"));

        let spec = app.container_spec(args);

        //self.log_execute_info(&spec);

        let engine = self.0;

        let image_path = build_image(self, app.container_image());

        let mut options = format!("--bind {}:/w --pwd /w", working_dir);

        let t = Telemetry::new(working_dir);

        if let Some(scratch) = &spec.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            options.push_str(&format!(" --bind {d}:/{scratch}"));
            fs::create_dir_all(&d)?;
        }

        let command = if let Some(entrypoint) = &spec.entrypoint {
            util::Command::new(engine)
                .arg("exec")
                .args(options.split(' '))
                .arg(image_path)
                .arg(entrypoint)
        } else {
            util::Command::new(engine)
                .arg("run")
                .args(options.split(' '))
                .arg(image_path)
        }
        .args(spec.args.clone())
        .live();

        let result = command.call();

        // println!("{}", result.stdout.bright_black());
        // eprintln!("{}", result.stderr.bright_red());

        let logs = format!(
            "{command}\nprocess success: {}\n{}\n{}\n{}\n",
            result.success, result.stdout, result.stderr, result.stderr
        );

        fs::write(t.log_file_name(), logs)?;

        if !result.success {
            eprintln!(
                "{}",
                "Container {engine} exited with non-zero status"
                    .bright_red()
                    .bold()
            );
            return Err(anyhow::anyhow!(
                "Docker container exited with non-zero status"
            ));
        }

        println!(
            "{}",
            format!(
                "The exact command line used and full log saved into {:?}\nScratch dir for this run is: {:?}\n",
                t.log_file_name(), t.scratch_dir()
            )
            .blue()
            .dim()
        );

        Ok(())
    }
}

fn build_image(engine: &HpcEngine, image: &str) -> Utf8PathBuf {
    let image_path = hpc_image_path(image);
    if !image_path.exists() {
        println!("Could not find {}, rebuilding...", image_path.green());
        Command::new(engine.0)
            .args(["pull", image_path.as_str(), &format!("docker://{}", image)])
            .live()
            .exec()
            .expect("error building image");
    }

    image_path
}

fn hpc_images_root() -> Utf8PathBuf {
    let root = dirs::cache_root().join("hpc");
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn hpc_image_path(image_path: &str) -> Utf8PathBuf {
    hpc_images_root().join(format!("{}.sif", image_path.replace("/", "-")))
}
