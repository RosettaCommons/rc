mod app;
mod config;
mod driver;
mod engine;
mod telemetry;
mod util;

use anyhow::{Result, anyhow};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use yansi::Paint;

use crate::{app::App, config::config_show, driver::install, engine::ContainerEngine};

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
    command: Commands,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clean an app installation
    Clean {
        /// The app to clean
        #[arg(value_enum, required_unless_present = "all", conflicts_with = "all")]
        app: Option<App>,

        #[arg(short, long, conflicts_with = "app")]
        all: bool,

        #[arg(short = 'e', long, conflicts_with = "all")]
        container_engine: Option<ContainerEngine>,
    },

    /// Install an app
    Install {
        /// The app to install
        #[arg(value_enum)]
        app: App,

        #[arg(short = 'e', long)]
        container_engine: ContainerEngine,
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
        container_engine: ContainerEngine,
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
    // /// Set a configuration value
    // Set {
    //     /// Dotted key path, e.g. `cache.root`
    //     key: String,

    //     /// Value to set (stringly-typed; you parse/validate per key)
    //     value: String,
    // },

    // /// Remove a configuration override (fall back to defaults)
    // Unset {
    //     /// Dotted key path, e.g. `cache.root`
    //     key: String,
    // },

    // /// Open the config file in $EDITOR
    // Edit,

    // /// Print the config file path
    // Path,
}

#[derive(clap::Args, Debug)]
struct ConfigShowArgs {
    /// Output as JSON (useful for scripting)
    #[arg(long)]
    json: bool,
    // /// Output as TOML (optional; pick what you support)
    // #[arg(long, conflicts_with = "json")]
    // toml: bool,

    // /// Include where each value came from (defaults/env/file)
    // #[arg(long)]
    // origin: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Running in verbose mode");
        println!("Args: {:#?}", args);
    }

    match args.command {
        Commands::Clean {
            app,
            all: _,
            container_engine,
        } => driver::clean(app, container_engine),

        Commands::Install {
            app,
            container_engine,
        } => install(app, container_engine),

        Commands::Run {
            app,
            args: app_args,
            container_engine,
            working_dir,
        } => {
            let working_dir = working_dir
                .unwrap_or_else(|| Utf8PathBuf::from("."))
                .canonicalize()
                .map_err(|_| anyhow!("Specified working directory does not exist".red()))?;

            let working_dir = Utf8PathBuf::try_from(working_dir)
                .map_err(|_| anyhow!("Working dir path contains invalid UTF-8".red()))?;

            driver::run(app.spec(), app_args, container_engine, working_dir)
        }
        Commands::Config { config_command } => match config_command {
            ConfigCmd::Show(show_args) => config_show(show_args.json),
            ConfigCmd::Get { .. } => unimplemented!(),
            // ConfigCmd::Set(_) => unimplemented!(),
            // ConfigCmd::Unset(_) => unimplemented!(),
            // ConfigCmd::Edit => unimplemented!(),
            // ConfigCmd::Path => unimplemented!(),
        },
        // None => {
        //     eprintln!("Error: No command specified");
        //     eprintln!("Use --help to see available commands");
        //     process::exit(1);
        // }
    }
}
