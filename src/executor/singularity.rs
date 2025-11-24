use crate::executor::Executor;

use anyhow::Result;

impl Executor {
    pub(crate) fn execute_with_singularity(&self) -> Result<()> {
        println!(
            "Running singularity container: {} working directory: {:?}",
            self.image.0, self.working_dir
        );
        todo!("Implement execute_with_singularity");
    }
}
