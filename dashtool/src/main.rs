use std::{fs, process::Command, str::FromStr, sync::Arc};

use anyhow::anyhow;
use dashtool::{
    build::build,
    error::Error,
    plugins::{file::FilePlugin, sql::SqlPlugin, Config, Plugin},
    workflow::workflow,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
    /// Set file for Argo workflow definition
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Workflow,
    Apply,
}

static DASHTOOL_CONFIG: &str = "dashtool.json";
static OUTPUT_FILE: &str = "argo/workflow.yaml";
static DAG_DIR: &str = ".dashtool/dags";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let output = args.file.unwrap_or(OUTPUT_FILE.to_owned());

    fs::create_dir_all(DAG_DIR).ok();
    fs::create_dir_all(
        std::path::Path::new(&output)
            .parent()
            .ok_or(Error::Anyhow(anyhow!("Output file cannot be a directory")))?,
    )
    .ok();
    #[cfg(not(target_arch = "wasm32"))]
    fs::create_dir_all(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool",
    )?;

    let config_json = fs::read_to_string(DASHTOOL_CONFIG)?;
    let config: Config = serde_json::from_str(&shellexpand::env(&config_json)?)?;

    let plugin: Arc<dyn Plugin> = match config {
        Config::Sql(sql_config) => {
            Ok::<_, Error>(Arc::new(SqlPlugin::new(sql_config).await?) as Arc<dyn Plugin>)
        }
        Config::File(file_config) => {
            Ok::<_, Error>(Arc::new(FilePlugin::new(file_config).await?) as Arc<dyn Plugin>)
        }
    }?;

    match args.commands {
        Commands::Build => build(plugin).await,
        Commands::Workflow => workflow(plugin, &output),
        Commands::Apply => {
            if cfg!(target_os = "windows") {
                Command::new("kubectl")
                    .args(["apply", "-f", &output])
                    .output()
                    .and_then(|x| {
                        if x.status.success() {
                            Ok(())
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Failed to apply Argo workflow to Kubernetes cluster",
                            ))
                        }
                    })
                    .map_err(Error::from)
            } else {
                Command::new("kubectl")
                    .args(["apply", "-f", &output])
                    .output()
                    .and_then(|x| {
                        if x.status.success() {
                            Ok(())
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Failed to apply Argo workflow to Kubernetes cluster",
                            ))
                        }
                    })
                    .map_err(Error::from)
            }
        }
    }
}
