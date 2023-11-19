use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{
    CronWorkflowBuilder, CronWorkflowSpecBuilder, DagTaskBuilder, DagTemplateBuilder,
    IoArgoprojWorkflowV1alpha1Template, TemplateBuilder, WorkflowSpecBuilder,
};
use git2::Repository;
use petgraph::Direction;

use crate::{
    dag::{get_dag, Node},
    error::Error,
    git::branch,
    plugins::Plugin,
};

use self::template::{iceberg_template, singer_template};

mod template;

static API_VERSION: &str = "argoproj.io/v1alpha1";

pub fn workflow(plugin: Arc<dyn Plugin>) -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let branch = branch(&repo)?;

    let dag = get_dag(&branch)?;

    let mut templates: HashMap<String, IoArgoprojWorkflowV1alpha1Template> =
        HashMap::from_iter(vec![("refresh".to_string(), iceberg_template(&*plugin)?)]);

    let tasks = dag
        .dag
        .node_indices()
        .map(|index| {
            let node = &dag.dag[index];
            let task = match node {
                Node::Singer(node) => {
                    templates
                        .entry(node.target.image.clone())
                        .or_insert_with(|| singer_template(&node, &*plugin).unwrap());
                    DagTaskBuilder::default()
                        .name(node.identifier.clone())
                        .template(Some(node.target.image.clone()))
                        .build()
                }
                Node::Tabular(node) => DagTaskBuilder::default()
                    .name(node.identifier.clone())
                    .template(Some("refresh".to_string()))
                    .dependencies(
                        dag.dag
                            .neighbors_directed(index, Direction::Incoming)
                            .into_iter()
                            .map(|x| dag.dag[x].identifier().to_owned())
                            .collect(),
                    )
                    .build(),
            }?;
            Ok::<_, Error>(task)
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let dag_template = TemplateBuilder::default()
        .name(Some("dashtool".to_string()))
        .dag(Some(DagTemplateBuilder::default().tasks(tasks).build()?))
        .build()?;

    templates.insert("dag".to_string(), dag_template);

    let workflow = CronWorkflowBuilder::default()
        .api_version(Some(API_VERSION.to_string()))
        .kind(Some("Workflow".to_string()))
        .spec(
            CronWorkflowSpecBuilder::default()
                .workflow_spec(
                    WorkflowSpecBuilder::default()
                        .entrypoint(Some("dashtool".to_string()))
                        .templates(templates.into_values().collect())
                        .build()?,
                )
                .build()?,
        )
        .build()?;

    let yaml = serde_yaml::to_string(&workflow)?;

    fs::write("kubernetes/workflow.yaml", yaml)?;

    Ok(())
}
