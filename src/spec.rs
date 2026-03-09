use std::{borrow::Cow, collections::HashMap};

use camino::Utf8Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MountRole {
    WorkingDir,
    Scratch,
}

// ContainerConfig or ContainerExecConfig
pub struct ContainerConfig {
    //pub image: Image,
    pub args: Vec<String>,
    pub mounts: HashMap<MountRole, String>,
    pub entrypoint: Option<String>,
}

pub struct NativeRunSpec {
    pub pixi: Cow<'static, str>,
    pub args: Vec<String>,
}

pub trait AppSpec {
    /// Docker image name — also the source for Singularity/Apptainer .sif builds.
    /// The only required method. Single source of truth for the container image.
    fn container_image(&self) -> &'static str;

    /// Pixi TOML recipe for native execution.
    /// `None` (default) means this app does not support native execution.
    fn pixi_recipe(&self) -> Option<&'static str> {
        None
    }

    fn container_spec(&self, args: Vec<String>) -> ContainerConfig;
    // {
    //     ContainerRunSpec::new(self.container_image(), args).working_dir("/w")
    // }

    fn native_spec(&self, args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec;
}

impl ContainerConfig {
    pub fn new(
        // image: impl Into<String>,
        args: Vec<String>,
    ) -> Self {
        Self {
            //image: Image(image.into()),
            args,
            mounts: HashMap::new(),
            entrypoint: None,
        }
    }
    pub fn with_prefixed_args<I1, I2, S1, S2>(
        // image: impl Into<String>,
        prefixes: I1,
        args: I2,
    ) -> Self
    where
        I1: IntoIterator<Item = S1>,
        I2: IntoIterator<Item = S2>,
        S1: Into<String>,
        S2: Into<String>,
    {
        let full_args: Vec<String> = prefixes
            .into_iter()
            .map(Into::into)
            .chain(args.into_iter().map(Into::into))
            .collect();

        Self {
            // image: Image(image.into()),
            args: full_args,
            mounts: HashMap::new(),
            entrypoint: None,
        }
    }

    pub fn scratch(mut self, p: impl Into<String>) -> Self {
        self.mounts.insert(MountRole::Scratch, p.into());
        self
    }
    pub fn working_dir(mut self, p: impl Into<String>) -> Self {
        self.mounts.insert(MountRole::WorkingDir, p.into());
        self
    }
    pub fn entrypoint(mut self, p: impl Into<String>) -> Self {
        self.entrypoint = Some(p.into());
        self
    }
}

impl NativeRunSpec {
    pub fn new(pixi: impl Into<Cow<'static, str>>, args: Vec<String>) -> Self {
        Self {
            pixi: pixi.into(),
            args,
        }
    }
    // pub fn pixi(mut self, p: impl Into<Cow<'static, str>>) -> Self {
    //     self.pixi = p.into();
    //     self
    // }
}

// impl RunSpec {
//     pub fn new(container: ContainerRunSpec, native: Option<NativeRunSpec>) -> Self {
//         Self { container, native }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_prefixed_args() {
        let prefixes = vec!["--prefix1", "--prefix2"];
        let args = vec!["arg1", "arg2", "arg3"];

        let spec = ContainerConfig::with_prefixed_args(prefixes.clone(), args.clone());

        assert_eq!(
            spec.args,
            vec!["--prefix1", "--prefix2", "arg1", "arg2", "arg3"]
        );
        assert_eq!(spec.mounts.len(), 0);
    }

    #[test]
    fn test_with_prefixed_args_empty_prefixes() {
        let prefixes: Vec<String> = vec![];
        let args = vec!["arg1".to_string(), "arg2".to_string()];

        let spec = ContainerConfig::with_prefixed_args(prefixes, args.clone());

        assert_eq!(spec.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_with_prefixed_args_empty_args() {
        let prefixes = vec!["--flag".to_string()];
        let args: Vec<String> = vec![];

        let spec = ContainerConfig::with_prefixed_args(prefixes.clone(), args);

        assert_eq!(spec.args.len(), 1);
        assert_eq!(spec.args[0], "--flag");
    }

    #[test]
    fn test_with_prefixed_args_both_empty() {
        let prefixes: Vec<String> = vec![];
        let args: Vec<String> = vec![];

        let spec = ContainerConfig::with_prefixed_args(prefixes, args);

        assert_eq!(spec.args.len(), 0);
        assert!(spec.mounts.is_empty());
    }
}
