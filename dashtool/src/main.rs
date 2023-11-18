use std::{fs, str::FromStr};

use anyhow::anyhow;
use dashtool::{build::build, error::Error, rebuild::rebuild, workflow::workflow};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Workflow,
    Rebuild,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    fs::create_dir_all(".dashtool/dags").ok();
    fs::create_dir_all("kubernetes").ok();
    #[cfg(not(target_arch = "wasm32"))]
    fs::create_dir_all(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool",
    )?;

    match args.commands {
        Commands::Build => build().await,
        Commands::Workflow => workflow(),
        Commands::Rebuild => rebuild().await,
    }
}
