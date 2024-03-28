use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod image;
mod runner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short)]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Run(ExecArgs),

    /// This is to debug the image manifest getting
    Image(DebugArgs),
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

#[derive(Args, Debug)]
pub(crate) struct DebugArgs {
    /// Image to run
    image: String,
}

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("CLI: {cli:?}");

    match cli.command {
        Commands::Run(args) => {
            let is = image::ImageService::new(&args.image);
            is.get_image_manifest()?;
            runner::run_command(&args.command, &args.args)?;
            Ok(())
        }
        Commands::Image(args) => {
            let is = image::ImageService::new(&args.image);
            is.get_image_manifest()?;
            Ok(())
        }
    }
}
