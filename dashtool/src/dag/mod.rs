use std::{collections::HashMap, fs};

use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub enum Node {
    Tabular(Tabular),
    Singer(Singer),
}

#[derive(Serialize, Deserialize)]
pub struct Tabular {
    identifier: String,
}

impl Tabular {
    pub(crate) fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Singer {
    identifier: String,
    tap: String,
    target: SingerConfig,
}

impl Singer {
    pub(crate) fn new(identifier: &str, tap: String, target: SingerConfig) -> Self {
        Self {
            identifier: identifier.to_owned(),
            tap,
            target,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dag {
    singers: HashMap<String, String>,
    map: HashMap<String, NodeIndex>,
    dag: StableDiGraph<Node, ()>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            singers: HashMap::new(),
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    }
}

impl Dag {
    pub(crate) fn add_node(&mut self, node: Node) {
        let identifier = match &node {
            Node::Singer(singer) => {
                let identifier = singer.identifier.clone();
                for (_, stream) in &singer.target.streams {
                    self.singers.insert(stream.clone(), identifier.clone());
                }
                identifier
            }
            Node::Tabular(tab) => tab.identifier.clone(),
        };
        let idx = self.dag.add_node(node);
        self.map.insert(identifier, idx);
    }
    pub(crate) fn add_edge(&mut self, a: &str, b: &str) -> Result<(), Error> {
        let a = self
            .map
            .get(a)
            .cloned()
            .ok_or(Error::Text("Node not in graph.".to_string()))?;
        let b = match self.singers.get(b) {
            None => self
                .map
                .get(b)
                .cloned()
                .ok_or(Error::Text("Node not in graph.".to_string()))?,
            Some(ident) => self
                .map
                .get(ident)
                .cloned()
                .ok_or(Error::Text("Node not in graph.".to_string()))?,
        };

        self.dag.add_edge(a, b, ());
        Ok(())
    }
}

pub fn get_dag() -> Result<Dag, Error> {
    let dag = if fs::metadata(".dashtool/dag.json").is_ok() {
        let json = fs::read_to_string(".dashtool/dag.json")?;
        let dag: Dag = serde_json::from_str(&json)?;
        dag
    } else {
        Dag {
            singers: HashMap::new(),
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    };
    Ok(dag)
}
