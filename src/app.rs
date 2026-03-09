mod foundry;
mod ligandmpnn;
mod picap;
mod proteinmpnn;
mod proteinmpnn_script;
mod pyrosetta;
mod rfdiffusion;
mod rosetta;
mod score;

use clap::ValueEnum;

use crate::spec::AppSpec;

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display, strum::EnumIter)]
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
    #[value(aliases = ["proteinmpnn-script", "ProteinMPNN-Script"])]
    ProteinmpnnScript,

    /// Run the LigandMPNN command https://github.com/dauparas/LigandMPNN
    #[value(aliases = ["LigandMPNN"])]
    Ligandmpnn,

    /// Run the Foundry command https://github.com/RosettaCommons/foundry
    #[value(aliases = ["Foundry"])]
    Foundry,
    // /// Run the PiCAP/CAPSIF2 command https://github.com/Graylab/picap
    // #[value(aliases = ["PiCAP", "CAPSIF2"])]
    // Picap,
}

impl App {
    pub fn spec(self) -> &'static dyn AppSpec {
        match self {
            App::Score => &score::SCORE,
            App::Rosetta => &rosetta::ROSETTA,
            App::PyRosetta => &pyrosetta::PYROSETTA,
            App::Rfdiffusion => &rfdiffusion::RFDIFFUSION,
            App::Proteinmpnn => &proteinmpnn::PROTEINMPNN,
            App::ProteinmpnnScript => &proteinmpnn_script::PROTEINMPNN_SCRIPT,
            App::Ligandmpnn => &ligandmpnn::LIGANDMPNN,
            App::Foundry => &foundry::FOUNDRY,
            // App::Picap => &picap::PICAP,
            //_ => panic!("unimplementet app"),
        }
    }
}
