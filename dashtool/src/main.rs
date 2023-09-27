use std::{fs, str::FromStr};

use anyhow::anyhow;
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
    #[cfg(not(target_arch = "wasm32"))]
    fs::create_dir_all(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool",
    )?;

    match args.commands {
        Commands::Run => run().await,
        Commands::Init => init().await,
    }
}
