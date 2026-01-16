use std::{fs, path::PathBuf};

use crate::{
    app::{MountRole, RunSpec},
    executor::{Executor, Image, Telemetry},
    run,
    util::{self, Command, dirs},
};

use anyhow::Result;
use yansi::Paint;

struct HpcContainerEngine(String);

// cover HpcContainerEngine
//hpc_container_engine @ HpcContainerEngine(engine): &HpcContainerEngine,
//
impl Executor {
    pub(super) fn execute_with_hpc_container_engine(&self, spec: RunSpec) -> Result<()> {
        assert!(matches!(
            self.engine,
            run::ContainerEngine::Singularity | run::ContainerEngine::Apptainer
        ));

        //self.log_execute_info(&spec);

        let engine = HpcContainerEngine(self.engine.to_string());

        let image_path = self.build_image(&engine, &spec.container.image);

        let mut options = format!("--bind {}:/w --pwd /w", self.working_dir.display());

        let t = Telemetry::new(&self.working_dir);

        if let Some(scratch) = &spec.container.mounts.get(&MountRole::Scratch) {
            let d = t.scratch_dir();
            options.push_str(&format!(
                " --bind {}:/{scratch}",
                d.to_str().expect("path is not valid UTF-8")
            ));
            fs::create_dir_all(&d)?;
        }

        let command = util::Command::new(engine.0)
            .arg("run")
            .args(options.split(' '))
            .arg(image_path.to_string_lossy())
            .args(spec.container.args.clone())
            //.message("")
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
        image: &Image,
    ) -> PathBuf {
        let image_path = self.image_path(image);
        if !image_path.exists() {
            println!(
                "Could not find {}, rebuilding...",
                image_path.to_str().unwrap().green()
            );
            Command::new(engine)
                .args([
                    "pull",
                    image_path.to_str().unwrap(),
                    &format!("docker://{}", image.0),
                ])
                .live()
                .exec()
                .expect("error building image");
        }

        image_path
    }

    fn images_root(&self) -> PathBuf {
        let root = dirs::cache_root().join("hpc");
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn image_path(&self, Image(image_path): &Image) -> PathBuf {
        self.images_root()
            .join(format!("{}.sif", image_path.replace("/", "-")))
    }
}
