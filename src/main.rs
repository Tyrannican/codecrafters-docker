use std::io::Write;

use anyhow::{Context, Result};

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
}

#[derive(Args, Debug)]
struct RunArgs {
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
        Commands::Run(args) => {
            let command = args.command;
            let command_args = args.args;

            let exec = std::process::Command::new(&command)
                .args(&command_args)
                .output()
                .with_context(|| {
                    format!(
                        "Running command {:?} with arguments {:?}",
                        &command, &command_args
                    )
                })?;

            let mut stdout = std::io::stdout();
            let mut stderr = std::io::stderr();
            stdout
                .write_all(&exec.stdout)
                .context("command stdout response")?;
            stderr
                .write(&exec.stderr)
                .context("command stderr output")?;
        }
    }

    Ok(())
}
