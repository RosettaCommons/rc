use std::{env::home_dir, path::PathBuf};

use crate::executor::{Executor, HpcContainerEngine, Image};

use anyhow::Result;

// cover HpcContainerEngine

impl Executor {
    pub(super) fn execute_with_hpc_container_engine(
        &self,
        HpcContainerEngine(engine): HpcContainerEngine,
    ) -> Result<()> {
        println!(
            "Running {engine} container: {} working directory: {:?}",
            self.image.0, self.working_dir
        );
        todo!("Implement execute_with_hpc_container_engine");
    }

    //fn build_image(

    fn images_root(&self) -> PathBuf {
        let root = home_dir().unwrap().join(".cache/rosettacommons/rc");
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn image_path(&self, Image(image_name): Image) -> PathBuf {
        self.images_root().join(format!("{image_name}.sif"))
    }
}
