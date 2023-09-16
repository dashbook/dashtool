use std::{collections::HashMap, fs};

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
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    }
}

impl Dag {
    pub(crate) fn add_node_with_children(
        &mut self,
        node: Node,
        children: &[String],
    ) -> Result<(), Error> {
        let identifier = node.identifier.clone();
        let idx = self.dag.add_node(node);
        self.map.insert(identifier, idx);

        let opt: Option<Vec<NodeIndex>> =
            children.iter().map(|x| self.map.get(x).cloned()).collect();

        if let Some(indices) = opt {
            for child in indices {
                self.dag.add_edge(idx, child, ());
            }
            Ok(())
        } else {
            Err(Error::Text("Nodes not in graph.".to_string()))
        }
    }
    pub(crate) fn add_node(&mut self, node: Node) {
        let identifier = node.identifier.clone();
        let idx = self.dag.add_node(node);
        self.map.insert(identifier, idx);
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
            map: HashMap::new(),
            dag: StableDiGraph::new(),
        }
    };
    Ok(dag)
}
