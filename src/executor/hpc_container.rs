use std::{env::home_dir, path::PathBuf};

use crate::{
    executor::{Executor, HpcContainerEngine, Image},
    util::Command,
};

use anyhow::Result;
use yansi::Paint;

// cover HpcContainerEngine

impl Executor {
    pub(super) fn execute_with_hpc_container_engine(
        &self,
        hpc_container_engine @ HpcContainerEngine(engine): &HpcContainerEngine,
    ) -> Result<()> {
        println!(
            "Running {engine} container: {} working directory: {:?}",
            self.image.0, self.working_dir
        );

        let _image_path = self.build_image(hpc_container_engine, &self.image);

        todo!("Implement execute_with_hpc_container_engine");
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
                .call();
        }

        image_path
    }

    fn images_root(&self) -> PathBuf {
        let root = home_dir().unwrap().join(".cache/rosettacommons/rc");
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn image_path(&self, Image(image_path): &Image) -> PathBuf {
        self.images_root()
            .join(format!("{}.sif", image_path.replace("/", "-")))
    }
}
