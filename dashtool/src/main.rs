use std::{fs, str::FromStr, sync::Arc};

use anyhow::anyhow;
use dashtool::{
    build::build,
    error::Error,
    plugins::{sql::SqlPlugin, Config, Plugin},
    workflow::{workflow, WORKFLOW_DIR},
};

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

    let config_json = fs::read_to_string("dashtool.json")?;
    let config: Config = serde_json::from_str(&config_json)?;

    let plugin: Arc<dyn Plugin> = match config {
        Config::Sql(sql_config) => {
            Ok::<_, Error>(Arc::new(SqlPlugin::new(sql_config).await?) as Arc<dyn Plugin>)
        }
    }?;

    match args.commands {
        Commands::Build => build(plugin).await,
        Commands::Workflow => workflow(plugin),
    }
}
