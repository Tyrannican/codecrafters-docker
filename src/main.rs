use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod runner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Run(ExecArgs),
}

#[derive(Args, Debug)]
pub(crate) struct ExecArgs {
    /// Image to run
    image: String,

    /// Command to execute
    command: String,

    /// Command arguments
    args: Vec<String>,
}

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => runner::run_command(&args.command, &args.args),
    }
}
