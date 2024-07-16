use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

use itertools::Itertools;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

use self::identifier::FullIdentifier;

pub(crate) mod identifier;

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    Tabular(Tabular),
    Ingest(Ingest),
}

impl Node {
    pub(crate) fn identifier(&self) -> &FullIdentifier {
        match self {
            Node::Ingest(ingest) => &ingest.identifier,
            Node::Tabular(tab) => &tab.identifier,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabular {
    pub(crate) identifier: FullIdentifier,
    pub(crate) branch: String,
    pub(crate) query: String,
}

impl Tabular {
    pub(crate) fn new(identifier: &FullIdentifier, branch: &str, query: &String) -> Self {
        Self {
            identifier: identifier.clone(),
            branch: branch.to_owned(),
            query: query.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingest {
    pub(crate) identifier: FullIdentifier,
    pub(crate) branch: String,
    pub(crate) image: String,
    pub(crate) source: JsonValue,
    pub(crate) destination: JsonValue,
}

impl Ingest {
    pub(crate) fn new(
        identifier: &FullIdentifier,
        image: &str,
        source: JsonValue,
        destination: JsonValue,
        branch: &str,
    ) -> Self {
        Self {
            identifier: identifier.clone(),
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
    pub(crate) ingests: HashMap<String, Vec<String>>,
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
                self.ingests
                    .entry(identifier.catalog_name().clone() + "." + identifier.namespace_name())
                    .and_modify(|x| x.push(identifier.to_string()))
                    .or_insert(vec![identifier.to_string()]);
                identifier
            }
            Node::Tabular(tab) => tab.identifier.clone(),
        };
        match self.map.entry(identifier.to_string()) {
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
            .ok_or(Error::Text(format!("Node {} not in graph.", a)))?;

        let bs = match self.ingests.get(&b.split(".").take(2).join(".")) {
            None => vec![self
                .map
                .get(b)
                .cloned()
                .ok_or(Error::Text(format!("Node {} not in graph.", b)))?],
            Some(ingests) => ingests
                .iter()
                .map(|ident| {
                    self.map
                        .get(ident)
                        .cloned()
                        .ok_or(Error::Text(format!("Node {} not in graph.", ident)))
                })
                .collect::<Result<Vec<_>, Error>>()?,
        };

        for b in bs {
            self.dag.add_edge(b, a, ());
        }

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
