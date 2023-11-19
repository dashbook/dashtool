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

impl Node {
    pub(crate) fn identifier(&self) -> &str {
        match self {
            Node::Singer(singer) => &singer.identifier,
            Node::Tabular(tab) => &tab.identifier,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tabular {
    pub(crate) identifier: String,
    pub(crate) branch: String,
}

impl Tabular {
    pub(crate) fn new(identifier: &str, branch: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
            branch: branch.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Singer {
    pub(crate) identifier: String,
    pub(crate) branch: String,
    pub(crate) tap: String,
    pub(crate) target: SingerConfig,
}

impl Singer {
    pub(crate) fn new(identifier: &str, tap: String, target: SingerConfig, branch: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
            branch: branch.to_owned(),
            tap,
            target,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dag {
    pub(crate) singers: HashMap<String, String>,
    pub(crate) map: HashMap<String, NodeIndex>,
    pub(crate) dag: StableDiGraph<Node, ()>,
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

        self.dag.add_edge(b, a, ());
        Ok(())
    }
}

pub fn get_dag(branch: &str) -> Result<Dag, Error> {
    let path = ".dashtool/dags/".to_string() + branch + ".json";
    let dag = if fs::metadata(&path).is_ok() {
        let json = fs::read_to_string(&path)?;
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
