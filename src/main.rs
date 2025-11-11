use std::process;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use yansi::Paint;

/// A simple command line tool
#[derive(Parser, Debug)]
#[command(name = "rc")]
#[command(version)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a app with optional arguments
    Run {
        /// The app to run
        #[arg(value_enum)]
        app: RunApp,

        /// Optional arguments for the app
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
#[strum(serialize_all = "kebab-case")] // "lowercase"
enum RunApp {
    /// Run the Rosetta score command
    Score,

    /// Run the Rosetta docking command
    Docking,
}

type Handler = fn(Vec<String>) -> Result<()>;

impl RunApp {
    fn handler(self) -> Handler {
        match self {
            RunApp::Score => foo_score,
            RunApp::Docking => foo_docking,
        }
    }

    fn execute(self, args: Vec<String>) -> Result<()> {
        (self.handler())(args)
    }
}

fn foo_score(args: Vec<String>) -> Result<()> {
    println!("Running score command");
    if !args.is_empty() {
        println!("With arguments: {:?}", args);
    }
    Ok(())
}

fn foo_docking(args: Vec<String>) -> Result<()> {
    println!("Running docking command");
    if !args.is_empty() {
        println!("With arguments: {:?}", args);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Running in verbose mode");
        println!("Args: {:#?}", args);
    }

    match &args.command {
        Some(Commands::Run {
            app,
            args: app_args,
        }) => {
            println!("Running app: {}", app.green());
            if !app_args.is_empty() {
                println!(
                    "With arguments: {}",
                    format!("{:?}", app_args).bright_blue()
                );
            }
            app.execute(app_args.clone())?;
            Ok(())
        }
        None => {
            eprintln!("Error: No command specified");
            eprintln!("Use --help to see available commands");
            process::exit(1);
        }
    }
}
