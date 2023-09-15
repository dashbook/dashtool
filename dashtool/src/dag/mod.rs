use std::{collections::HashMap, fs};

use iceberg_rust::catalog::identifier;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Node {
    identifier: String,
}

impl Node {
    pub(crate) fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dag {
    map: HashMap<String, NodeIndex>,
    dag: StableDiGraph<Node, ()>,
}

impl Dag {
    pub(crate) fn add_node(&mut self, node: Node, children: &[String]) {
        let identifier = node.identifier.clone();
        let idx = self.dag.add_node(node);
        self.map.insert(identifier, idx);

        let opt: Option<Vec<NodeIndex>> =
            children.iter().map(|x| self.map.get(x).cloned()).collect();

        if let Some(indices) = opt {
            for child in indices {
                self.dag.add_edge(idx, child, ());
            }
        }
    }
}

pub fn get_dag() -> Result<Dag, Error> {
    let dag = if fs::metadata(".dashtool/dag.json").is_ok() {
        let json = fs::read_to_string(".dashtool/dag.json")?;
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
