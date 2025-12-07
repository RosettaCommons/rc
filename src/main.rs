mod executor;
mod util;

use std::{path::PathBuf, process};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use yansi::Paint;

/// A command line tool to run various Rosetta applications
#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
#[strum(serialize_all = "lowercase")] //  "kebab-case"
enum ContainerEngine {
    Docker,
    Singularity,
    Apptainer,
    None,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clean an app installation
    Clean {
        /// The app to clean
        #[arg(value_enum)]
        app: App,
    },

    /// Install an app
    Install {
        /// The app to install
        #[arg(value_enum)]
        app: App,
    },

    /// Run an app with optional arguments
    Run {
        /// The app to run
        #[arg(value_enum)]
        app: App,

        /// Optional arguments for the app
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Input directory path
        #[arg(short, long)]
        working_dir: Option<PathBuf>,

        #[arg(short = 'e', long, default_value = "docker")]
        container_engine: ContainerEngine,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
#[clap(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")] //  "kebab-case"
enum App {
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
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Running in verbose mode");
        println!("Args: {:#?}", args);
    }

    match &args.command {
        Some(Commands::Clean { app }) => {
            println!("Cleaning app: {}", app.red());
            todo!();
        }
        Some(Commands::Install { app }) => {
            println!("Cleaning app installation: {}", app.bright_green());
            todo!();
        }
        Some(Commands::Run {
            app,
            args: app_args,
            container_engine,
            working_dir,
        }) => {
            let working_dir = working_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("."))
                .canonicalize()
                .unwrap();

            executor::run(app, app_args.clone(), container_engine, working_dir)
        }
        None => {
            eprintln!("Error: No command specified");
            eprintln!("Use --help to see available commands");
            process::exit(1);
        }
    }
}
