use anyhow::Result;
//use clap::{Args, Parser, Subcommand};

mod image;
mod runner;

//#[derive(Parser, Debug)]
//#[command(version, about, long_about = None)]
//pub(crate) struct Cli {
//    #[command(subcommand)]
//    command: Commands,
//}
//
//#[derive(Subcommand, Debug)]
//pub(crate) enum Commands {
//    Run(ExecArgs),
//}
//
//#[derive(Args, Debug)]
//pub(crate) struct ExecArgs {
//    /// Image to run
//    image: String,
//
//    /// Command to execute
//    #[arg(value_delimiter = ' ')]
//    command: Vec<String>,
//}

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
// TODO: Get Clap to understand -c as an arg and not a command
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let command = &args[1];
    let image = &args[2];
    let exec_command = &args[3];
    let exec_command_args = &args[4..];

    match command.as_ref() {
        "run" => {
            //
            runner::run_command(image, exec_command, exec_command_args)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
