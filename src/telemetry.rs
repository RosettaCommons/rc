use camino::{Utf8Path, Utf8PathBuf};

pub struct Telemetry {
    working_dir: Utf8PathBuf,
    prefix: String,
}

impl Telemetry {
    pub fn new(working_dir: &Utf8Path) -> Self {
        let mut i: u32 = 0;
        loop {
            let prefix = format!(".{i:04}.rc");
            i += 1;

            let r = Telemetry {
                working_dir: working_dir.to_path_buf(),
                prefix,
            };

            if r.log_file_name().exists() || r.scratch_dir().exists() {
                continue;
            }

            break r;
        }
    }

    pub fn log_file_name(&self) -> Utf8PathBuf {
        self.working_dir.join(format!("{}.log", self.prefix))
    }

    pub fn scratch_dir(&self) -> Utf8PathBuf {
        self.working_dir.join(format!("rc.scratch/{}", self.prefix))
    }
}
