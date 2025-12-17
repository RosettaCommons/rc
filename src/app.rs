use std::collections::HashMap;

use clap::ValueEnum;

mod ligandmpnn;
mod picap;
mod proteinmpnn;
mod proteinmpnn_script;
mod pyrosetta;
mod rfdiffusion;
mod rosetta;
mod score;

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

pub struct RunSpec {
    pub image: Image,
    pub args: Vec<String>,
    //pub scratch: Option<PathBuf>,
    pub mounts: HashMap<MountRole, String>,
}

impl RunSpec {
    pub fn new(image: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            image: Image(image.into()),
            args,
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

impl App {
    pub fn run_spec(self, app_args: Vec<String>) -> RunSpec {
        match self {
            App::Score => score::spec(app_args),
            App::Rosetta => rosetta::spec(app_args),
            App::PyRosetta => pyrosetta::spec(app_args),
            App::Rfdiffusion => rfdiffusion::spec(app_args),
            App::Proteinmpnn => proteinmpnn::spec(app_args),
            App::ProteinmpnnScript => proteinmpnn_script::spec(app_args),
            App::Ligandmpnn => ligandmpnn::spec(app_args),
            App::Picap => picap::spec(app_args),
        }
    }
}
