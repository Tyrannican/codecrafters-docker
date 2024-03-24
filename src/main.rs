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
    println!("Logs from your program will appear here!");
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => {
            println!("The run args: {args:?}");
        }
    }
    //let args: Vec<_> = std::env::args().collect();
    //let command = &args[3];
    //let command_args = &args[4..];
    //let output = std::process::Command::new(command)
    //    .args(command_args)
    //    .output()
    //    .with_context(|| {
    //        format!(
    //            "Tried to run '{}' with arguments {:?}",
    //            command, command_args
    //        )
    //    })?;

    //if output.status.success() {
    //    let std_out = std::str::from_utf8(&output.stdout)?;
    //    println!("{}", std_out);
    //} else {
    //    std::process::exit(1);
    //}

    Ok(())
}
