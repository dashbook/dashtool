use anyhow::anyhow;
use std::{fs, path::Path};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    dag::{Dag, Node, Singer, Tabular},
    error::Error,
    sql::find_relations,
};

pub async fn init() -> Result<(), Error> {
    if fs::metadata(".dashtool/dag.json").is_ok() {
        return Err(Error::Text("Dag is already initialized.".to_string()));
    }

    let mut dag = Dag::new();

    traverse(&Path::new("."), &mut dag, add_node)?;
    traverse(&Path::new("."), &mut dag, add_edge)?;

    let json = serde_json::to_string(&dag)?;

    fs::write(".dashtool/dag.json", json)?;

    Ok(())
}

fn traverse<P: AsRef<Path>>(
    path: &P,
    dag: &mut Dag,
    f: fn(&str, &mut Dag) -> Result<(), Error>,
) -> Result<(), Error> {
    for res in fs::read_dir(path)? {
        let entry = res?;
        if entry.file_type()?.is_dir() {
            traverse(path, dag, f)?
        } else if entry.file_type()?.is_file() {
            let path = entry.path();
            let path = path
                .to_str()
                .ok_or(Error::Text("Couldn't convert path to string.".to_string()))?;
            f(path, dag)?
        }
    }
    Ok(())
}

fn add_node(path: &str, dag: &mut Dag) -> Result<(), Error> {
    if path.ends_with("target.json") {
        let parent = Path::new(path)
            .parent()
            .ok_or(Error::Anyhow(anyhow!(
                "target.json must be inside a subfolder."
            )))?
            .to_str()
            .ok_or(Error::Anyhow(anyhow!("Failed to convert path to string.")))?;
        let tap_path = parent.to_string() + "/tap.json";
        let tap = fs::read_to_string(&tap_path)?;
        let target_json = fs::read_to_string(path)?;
        let target: SingerConfig = serde_json::from_str(&target_json)?;
        dag.add_node(Node::Singer(Singer::new(&parent, tap, target)))
    } else if path.ends_with(".sql") {
        dag.add_node(Node::Tabular(Tabular::new(
            &path
                .trim_end_matches(".sql")
                .trim_start_matches("/")
                .replace("/", "."),
        )))
    }
    Ok(())
}

fn add_edge(path: &str, dag: &mut Dag) -> Result<(), Error> {
    if path.ends_with(".sql") {
        let sql = fs::read_to_string(path)?;
        let relations = find_relations(&sql)?;
        let name = &path
            .trim_end_matches(".sql")
            .trim_start_matches("/")
            .replace("/", ".");
        for child in relations {
            dag.add_edge(&name, &child)?;
        }
    }
    Ok(())
}
