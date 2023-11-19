use std::{fs, str::FromStr, sync::Arc};

use anyhow::anyhow;
use dashtool::{
    build::build,
    error::Error,
    plugins::{dashbook::DashbookPlugin, Plugin},
    workflow::workflow,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,

    /// Plugin to execute. Available options are [dashbook, sql]. Defaults to dashbook.
    #[arg(short, long)]
    plugin: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Workflow,
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

    let plugin: Arc<dyn Plugin> = match args.plugin.as_deref() {
        Some("dashbook") | None => Ok(Arc::new(DashbookPlugin::new("dashtool.json").await?)),
        _ => Err(Error::Text("Unknown plugin".to_string())),
    }?;

    match args.commands {
        Commands::Build => build(plugin).await,
        Commands::Workflow => workflow(plugin),
    }
}
