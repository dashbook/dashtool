use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

pub(crate) mod identifier;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabular {
    pub(crate) identifier: String,
    pub(crate) branch: String,
    pub(crate) query: String,
}

impl Tabular {
    pub(crate) fn new(identifier: &str, branch: &str, query: &String) -> Self {
        Self {
            identifier: identifier.to_owned(),
            branch: branch.to_owned(),
            query: query.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Singer {
    pub(crate) identifier: String,
    pub(crate) branch: String,
    pub(crate) tap: JsonValue,
    pub(crate) target: JsonValue,
}

impl Singer {
    pub(crate) fn new(identifier: &str, tap: JsonValue, target: JsonValue, branch: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
            branch: branch.to_owned(),
            tap,
            target,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dag {
    pub(crate) map: HashMap<String, NodeIndex>,
    pub(crate) dag: StableDiGraph<Node, ()>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    }
}

impl Dag {
    pub(crate) fn add_node(&mut self, node: Node) {
        match self.map.entry(node.identifier().to_string()) {
            Entry::Vacant(entry) => {
                let idx = self.dag.add_node(node);
                entry.insert(idx);
            }
            Entry::Occupied(entry) => {
                let idx = entry.get();
                self.dag[*idx] = node;
            }
        };
    }

    pub(crate) fn add_edge(&mut self, a: &str, b: &str) -> Result<(), Error> {
        let a = self
            .map
            .get(a)
            .cloned()
            .ok_or(Error::Text("Node not in graph.".to_string()))?;

        let b = self
            .map
            .get(b)
            .cloned()
            .ok_or(Error::Text("Node not in graph.".to_string()))?;

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
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    };
    Ok(dag)
}
