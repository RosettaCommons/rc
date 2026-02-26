mod app;
mod executor;
mod run;
mod util;

use std::process;

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use yansi::Paint;

use crate::app::App;

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
        working_dir: Option<Utf8PathBuf>,

        #[arg(short = 'e', long, default_value = "docker")]
        container_engine: run::ContainerEngine,
    },

    Config {
        #[command(subcommand)]
        config_command: ConfigCmd,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCmd {
    /// Show the effective configuration
    Show(ConfigShowArgs),

    /// Get a single configuration value
    Get {
        /// Dotted key path, e.g. `cache.root`
        key: String,

        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
    },

    /// Set a configuration value
    Set {
        /// Dotted key path, e.g. `cache.root`
        key: String,

        /// Value to set (stringly-typed; you parse/validate per key)
        value: String,
    },

    /// Remove a configuration override (fall back to defaults)
    Unset {
        /// Dotted key path, e.g. `cache.root`
        key: String,
    },

    /// Open the config file in $EDITOR
    Edit,

    /// Print the config file path
    Path,
}

#[derive(clap::Args, Debug)]
struct ConfigShowArgs {
    /// Output as JSON (useful for scripting)
    #[arg(long)]
    json: bool,

    /// Output as TOML (optional; pick what you support)
    #[arg(long, conflicts_with = "json")]
    toml: bool,

    /// Include where each value came from (defaults/env/file)
    #[arg(long)]
    origin: bool,
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
                .unwrap_or_else(|| Utf8PathBuf::from("."))
                .canonicalize()
                .unwrap_or_else(|_| {
                    panic!("{}", "Specified working directory does not exist".red())
                });
            let working_dir = Utf8PathBuf::try_from(working_dir)
                .unwrap_or_else(|_| panic!("{}", "Working dir path contains invalid UTF-8".red()));

            run::run(app, app_args.clone(), container_engine, working_dir)
        }
        Some(Commands::Config { config_command: _ }) => {
            todo!();
        }

        None => {
            eprintln!("Error: No command specified");
            eprintln!("Use --help to see available commands");
            process::exit(1);
        }
    }
}
