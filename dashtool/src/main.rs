use std::{fs, str::FromStr, sync::Arc};

use anyhow::anyhow;
use dashtool::{
    build::build,
    error::Error,
    plugins::{sql::SqlPlugin, Plugin},
    workflow::{workflow, WORKFLOW_DIR},
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
    fs::create_dir_all(WORKFLOW_DIR).ok();
    #[cfg(not(target_arch = "wasm32"))]
    fs::create_dir_all(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool",
    )?;

    let plugin: Arc<dyn Plugin> = match args.plugin.as_deref() {
        Some("sql") => Ok(Arc::new(SqlPlugin::new("dashtool.json").await?) as Arc<dyn Plugin>),
        _ => Err(Error::Text("Unknown plugin".to_string())),
    }?;

    match args.commands {
        Commands::Build => build(plugin).await,
        Commands::Workflow => workflow(plugin),
    }
}
