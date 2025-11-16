use assert_fs::TempDir;
use assert_fs::prelude::*;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const SHIM_SCRIPT: &str = r#"#!/usr/bin/env bash
echo "$0 $@" >> "$TEST_INVOCATIONS_LOG"
exit 0
"#;

/// Creates a temp dir with container CLI shims and utilities to use them in tests.
pub struct ContainerPathShim {
    _dir: TempDir,
    bin: PathBuf,
}

impl ContainerPathShim {
    pub const BIN_SHIMS: [&str; 3] = ["docker", "singularity", "apptainer"];

    pub fn new() -> Self {
        let root = std::path::PathBuf::from("target/fixtures");
        std::fs::create_dir_all(&root).expect("create fixtures dir");
        //eprintln!("root:{root:?}");
        let dir = TempDir::new_in(root).expect("create temp dir");

        // add a label directory inside for readability
        let label = dir.child("bin");
        label.create_dir_all().unwrap();
        Self {
            _dir: dir,
            bin: label.to_path_buf(),
        }
    }

    /// Directory that contains the shims (add your own tools here if desired).
    pub fn bin_dir(&self) -> PathBuf {
        // we’ll store shims in the top-level temp dir for easy PATH use
        self.bin.clone()
    }

    /// Write all three shims (`docker`, `singularity`, `apptainer`) into the bin dir.
    pub fn install_all(&self) -> PathBuf {
        for name in Self::BIN_SHIMS {
            self.install(name);
        }
        self.bin.clone()
    }

    /// Write a single shim `<name>` into the bin dir.
    pub fn install(&self, name: &str) -> PathBuf {
        let path = self.bin_dir().join(name);
        write_executable(&path, SHIM_SCRIPT);
        self.bin.clone()
    }

    /// Returns a shell snippet that constrains PATH to only this directory.
    ///
    /// Example use in a test:
    /// `let cmd = format!("{} && my-runner ...", shim.export_path_cmd());`
    #[allow(dead_code)]
    pub fn export_path_cmd(&self) -> String {
        format!("export PATH={}:$PATH", self.bin_dir().display())
    }

    /// Returns env overrides you can pass to `std::process::Command` directly.
    /// This mirrors what `export_path_cmd()` would do in a shell.
    pub fn env_overrides(&self) -> [(&'static str, String); 1] {
        let current_path = std::env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", self.bin_dir().display(), current_path);
        [("PATH", new_path)]
    }
}

fn write_executable(path: &Path, content: &str) {
    fs::write(path, content).expect("write shim");
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shims_are_installed() {
        let shim = ContainerPathShim::new();
        let bin_dir = shim.install_all();

        for s in ContainerPathShim::BIN_SHIMS {
            let shim_path = bin_dir.join(s);
            assert!(shim_path.exists(), "{} shim should exist", s);
        }
    }

    #[test]
    fn test_single_shim_installation() {
        for s in ContainerPathShim::BIN_SHIMS {
            let shim = ContainerPathShim::new();
            let bin_dir = shim.install(s);

            let shim_path = bin_dir.join(s);
            assert!(shim_path.exists(), "{} shim should exist", s);

            let metadata = fs::metadata(bin_dir.join(s)).expect("shim metadata");
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            // Check that the executable bit is set (0o100 for owner execute)
            assert!(
                mode & 0o100 != 0,
                "{} shim should be executable (mode: {:o})",
                s,
                mode
            );
        }
    }
}
