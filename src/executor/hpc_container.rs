use std::fs;

use camino::Utf8PathBuf;

use crate::{
    executor::{Executor, Telemetry},
    run,
    spec::{ContainerConfig, MountRole},
    util::{self, Command, dirs},
};

use anyhow::Result;
use yansi::Paint;

struct HpcContainerEngine(String);

// cover HpcContainerEngine
//hpc_container_engine @ HpcContainerEngine(engine): &HpcContainerEngine,
//
impl Executor {
    pub(super) fn execute_with_hpc_container_engine(&self, spec: ContainerConfig) -> Result<()> {
        assert!(matches!(
            self.engine,
            run::ContainerEngine::Singularity | run::ContainerEngine::Apptainer
        ));

        //self.log_execute_info(&spec);

        let engine = HpcContainerEngine(self.engine.to_string());

        let image_path = self.build_image(&engine, self.app.spec().container_image());

        let mut options = format!("--bind {}:/w --pwd /w", self.working_dir);

        let t = Telemetry::new(&self.working_dir);

        if let Some(scratch) = &spec.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            options.push_str(&format!(" --bind {d}:/{scratch}"));
            fs::create_dir_all(&d)?;
        }

        let command = if let Some(entrypoint) = &spec.entrypoint {
            util::Command::new(engine.0)
                .arg("exec")
                .args(options.split(' '))
                .arg(image_path)
                .arg(entrypoint)
        } else {
            util::Command::new(engine.0)
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

    fn build_image(
        &self,
        HpcContainerEngine(engine): &HpcContainerEngine,
        image: &str,
    ) -> Utf8PathBuf {
        let image_path = Self::hpc_image_path(image);
        if !image_path.exists() {
            println!("Could not find {}, rebuilding...", image_path.green());
            Command::new(engine)
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
        Self::hpc_images_root().join(format!("{}.sif", image_path.replace("/", "-")))
    }
}
