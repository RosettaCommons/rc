mod foundry;
mod ligandmpnn;
mod picap;
mod proteinmpnn;
mod proteinmpnn_script;
mod pyrosetta;
mod rfdiffusion;
mod rosetta;
mod score;

use std::{borrow::Cow, collections::HashMap};

use camino::Utf8Path;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
#[clap(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")] //  "kebab-case"
pub enum App {
    /// Run the Rosetta score command
    Score,

    /// Run the Rosetta protocol
    Rosetta,

    /// Start python in env where PyRosetta is installed and execute script
    #[value(aliases = ["PyRosetta"])]
    PyRosetta,

    /// Run the RFdiffusion command https://github.com/RosettaCommons/RFdiffusion
    #[value(aliases = ["Rfdiffusion"])]
    Rfdiffusion,

    /// Run the ProteinMPNN command https://github.com/dauparas/ProteinMPNN
    #[value(aliases = ["ProteinMPNN"])]
    Proteinmpnn,

    /// Run the ProteinMPNN Script command https://github.com/dauparas/ProteinMPNN
    #[value(aliases = ["Proteinmpnn-script", "ProteinMPNN-Script"])]
    ProteinmpnnScript,

    /// Run the LigandMPNN command https://github.com/dauparas/LigandMPNN
    #[value(aliases = ["LigandMPNN"])]
    Ligandmpnn,

    /// Run the Foundry command https://github.com/RosettaCommons/foundry
    #[value(aliases = ["Foundry"])]
    Foundry,

    /// Run the PiCAP/CAPSIF2 command https://github.com/Graylab/picap
    #[value(aliases = ["PiCAP", "CAPSIF2"])]
    Picap,
}

pub struct Image(pub String);

// enum IoLayout {
//     Workdir(PathBuf),
//     InputOutput { input: PathBuf, output: PathBuf },
// }
// struct ContainerMounts {
//     io: IoLayout,
//     scratch: Option<PathBuf>,
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MountRole {
    WorkingDir,
    Scratch,
}

// enum InputControl {
//     /// The package expects a file path via a CLI flag (e.g. "--input").
//     FlagFile { flag: &'static str },
//     // /// The package expects a directory path via a CLI flag (e.g. "--input-dir").
//     // FlagDir { flag: &'static str },

//     // /// The package reads inputs from a fixed directory (typically inside the container).
//     // FixedDir { dir: PathBuf },

//     // /// The package expects a file path via a CLI argument.
//     // ArgFile { arg: String },
// }
// enum OutputControl {
//     /// Package writes outputs using a prefix provided via CLI flag
//     /// (e.g. "--out-prefix results/run1")
//     FlagPrefix { flag: &'static str },
//     // /// Package writes outputs into a directory provided via CLI flag
//     // /// (e.g. "--out-dir /results")
//     // FlagDir { flag: String },

//     // /// Package writes outputs into a fixed directory
//     // FixedDir { dir: PathBuf },
// }

pub struct ContainerRunSpec {
    pub image: Image,
    pub args: Vec<String>,
    pub mounts: HashMap<MountRole, String>,
}

pub struct NativeRunSpec {
    pub pixi: Cow<'static, str>,
    pub args: Vec<String>,
}

// pub struct RunSpec {
//     pub container: ContainerRunSpec,
//     pub native: Option<NativeRunSpec>,
// }

impl ContainerRunSpec {
    pub fn new(image: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            image: Image(image.into()),
            args,
            mounts: HashMap::new(),
        }
    }
    pub fn with_prefixed_args<I1, I2, S1, S2>(
        image: impl Into<String>,
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
            image: Image(image.into()),
            args: full_args,
            mounts: HashMap::new(),
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

impl App {
    pub fn container_spec(self, app_args: Vec<String>) -> ContainerRunSpec {
        match self {
            App::Score => score::container_spec(app_args),
            App::Rosetta => rosetta::container_spec(app_args),
            App::PyRosetta => pyrosetta::container_spec(app_args),
            App::Rfdiffusion => rfdiffusion::container_spec(app_args),
            App::Proteinmpnn => proteinmpnn::container_spec(app_args),
            App::ProteinmpnnScript => proteinmpnn_script::container_spec(app_args),
            App::Ligandmpnn => ligandmpnn::container_spec(app_args),
            App::Foundry => foundry::container_spec(app_args),
            App::Picap => picap::container_spec(app_args),
        }
    }

    pub fn native_spec(self, app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        match self {
            App::Score => todo!("not implemented"), // score::native_spec(app_args),
            App::Rosetta => todo!("not implemented"), // rosetta::native_spec(app_args),
            App::PyRosetta => todo!("not implemented"), // pyrosetta::native_spec(app_args),
            App::Rfdiffusion => rfdiffusion::native_spec(app_args, working_dir),
            App::Proteinmpnn => proteinmpnn::native_spec(app_args, working_dir),
            App::ProteinmpnnScript => todo!("not implemented"), // proteinmpnn_script::native_spec(app_args),
            App::Ligandmpnn => ligandmpnn::native_spec(app_args, working_dir),
            App::Foundry => foundry::native_spec(app_args, working_dir),
            App::Picap => todo!("not implemented"), // picap::native_spec(app_args),
        }
    }
}

// /// Dispatches a method call to the appropriate app module.
// /// Each App variant calls module::method(args) with the same name.
// macro_rules! dispatch_spec {
//     ($method:ident, $self:expr, $args:expr) => {
//         match $self {
//             App::Score => score::$method($args),
//             App::Rosetta => rosetta::$method($args),
//             App::PyRosetta => pyrosetta::$method($args),
//             App::Rfdiffusion => rfdiffusion::$method($args),
//             App::Proteinmpnn => proteinmpnn::$method($args),
//             App::ProteinmpnnScript => proteinmpnn_script::$method($args),
//             App::Ligandmpnn => ligandmpnn::$method($args),
//             App::Picap => picap::$method($args),
//         }
//     };
// }
// impl App {
//     pub fn container_spec(self, app_args: Vec<String>) -> ContainerRunSpec {
//         dispatch_spec!(container_spec, self, app_args)
//     }
//     pub fn native_spec(self, app_args: Vec<String>) -> Option<NativeRunSpec> {
//         dispatch_spec!(native_spec, self, app_args)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_prefixed_args() {
        let prefixes = vec!["--prefix1", "--prefix2"];
        let args = vec!["arg1", "arg2", "arg3"];

        let spec = ContainerRunSpec::with_prefixed_args(
            "test/image:latest",
            prefixes.clone(),
            args.clone(),
        );

        assert_eq!(spec.image.0, "test/image:latest");
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

        let spec = ContainerRunSpec::with_prefixed_args("test/image:v1", prefixes, args.clone());

        assert_eq!(spec.image.0, "test/image:v1");
        assert_eq!(spec.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_with_prefixed_args_empty_args() {
        let prefixes = vec!["--flag".to_string()];
        let args: Vec<String> = vec![];

        let spec = ContainerRunSpec::with_prefixed_args("test/image", prefixes.clone(), args);

        assert_eq!(spec.image.0, "test/image");
        assert_eq!(spec.args.len(), 1);
        assert_eq!(spec.args[0], "--flag");
    }

    #[test]
    fn test_with_prefixed_args_both_empty() {
        let prefixes: Vec<String> = vec![];
        let args: Vec<String> = vec![];

        let spec = ContainerRunSpec::with_prefixed_args("empty/image", prefixes, args);

        assert_eq!(spec.image.0, "empty/image");
        assert_eq!(spec.args.len(), 0);
        assert!(spec.mounts.is_empty());
    }
}
