use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

use anyhow::anyhow;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

pub(crate) mod identifier;

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    Tabular(Tabular),
    Ingest(Ingest),
}

impl Node {
    pub(crate) fn identifier(&self) -> &str {
        match self {
            Node::Ingest(ingest) => &ingest.identifier,
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
pub struct Ingest {
    pub(crate) identifier: String,
    pub(crate) branch: String,
    pub(crate) image: String,
    pub(crate) source: JsonValue,
    pub(crate) destination: JsonValue,
}

impl Ingest {
    pub(crate) fn new(
        identifier: &str,
        image: &str,
        source: JsonValue,
        destination: JsonValue,
        branch: &str,
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            branch: branch.to_owned(),
            image: image.to_owned(),
            source,
            destination,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct IngestConfig {
    pub(crate) image: String,
    pub(crate) source: JsonValue,
    pub(crate) destination: JsonValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dag {
    pub(crate) ingests: HashMap<String, String>,
    pub(crate) map: HashMap<String, NodeIndex>,
    pub(crate) dag: StableDiGraph<Node, ()>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            ingests: HashMap::new(),
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    }
}

impl Dag {
    pub(crate) fn add_node(&mut self, node: Node) -> Result<(), Error> {
        let identifier = match &node {
            Node::Ingest(ingest) => {
                let identifier = ingest.identifier.clone();
                let streams: HashMap<String, JsonValue> =
                    serde_json::from_value(ingest.destination["streams"].clone())
                        .expect("target.json must contain streams field.");
                for (_, config) in &streams {
                    let stream = if let JsonValue::String(stream) = &config["identifier"] {
                        Ok(stream)
                    } else {
                        Err(Error::Anyhow(anyhow!("Stream must have an identifier")))
                    }?;
                    self.ingests.insert(stream.clone(), identifier.clone());
                }
                identifier
            }
            Node::Tabular(tab) => tab.identifier.clone(),
        };
        match self.map.entry(identifier) {
            Entry::Vacant(entry) => {
                let idx = self.dag.add_node(node);
                entry.insert(idx);
            }
            Entry::Occupied(entry) => {
                let idx = entry.get();
                self.dag[*idx] = node;
            }
        };
        Ok(())
    }

    pub(crate) fn add_edge(&mut self, a: &str, b: &str) -> Result<(), Error> {
        let a = self
            .map
            .get(a)
            .cloned()
            .ok_or(Error::Text("Node not in graph.".to_string()))?;

        let b = match self.ingests.get(b) {
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
            ingests: HashMap::new(),
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    };
    Ok(dag)
}
