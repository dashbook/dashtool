use std::fs;

use dashtool::{error::Error, init::init, run::run};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,
    Init,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    fs::create_dir_all(".dashtool/dags").ok();

    match args.commands {
        Commands::Run => run().await,
        Commands::Init => init().await,
    }
}
