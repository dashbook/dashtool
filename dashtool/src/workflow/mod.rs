use std::{
    collections::{BTreeMap, HashMap},
    fs,
    sync::Arc,
};

use argo_workflow::schema::{
    ArgumentsBuilder, CronWorkflowBuilder, CronWorkflowSpecBuilder, DagTaskBuilder,
    DagTemplateBuilder, IoArgoprojWorkflowV1alpha1Template, ObjectMetaBuilder, ParameterBuilder,
    TemplateBuilder, WorkflowSpecBuilder,
};
use git2::Repository;
use k8s_openapi::api::core::v1::ConfigMap;
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
pub static WORKFLOW_DIR: &str = "argo";

pub fn workflow(plugin: Arc<dyn Plugin>) -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let branch = branch(&repo)?;

    let dag = get_dag(&branch)?;

    let mut templates: HashMap<String, IoArgoprojWorkflowV1alpha1Template> =
        HashMap::from_iter(vec![("refresh".to_string(), iceberg_template(&*plugin)?)]);

    let mut config_maps: HashMap<String, ConfigMap> = HashMap::new();

    let tasks = dag
        .dag
        .node_indices()
        .map(|index| {
            let node = &dag.dag[index];
            let task = match node {
                Node::Singer(node) => {
                    templates
                        .entry(serde_json::from_value::<String>(
                            node.target["image"].clone(),
                        )?)
                        .or_insert_with(|| singer_template(&node, &*plugin).unwrap());

                    let mut config_map = ConfigMap::default();
                    config_map.metadata.name = Some(
                        node.identifier
                            .replace(['/', ':', '_', '.'], "-")
                            .to_lowercase()
                            .to_owned()
                            + "-config-template",
                    );
                    config_map.data = Some(BTreeMap::from_iter(vec![
                        ("tap.json".to_owned(), serde_json::to_string(&node.tap)?),
                        (
                            "target.json".to_owned(),
                            serde_json::to_string(&node.target)?,
                        ),
                    ]));
                    config_maps.insert(
                        node.identifier
                            .replace(['/', ':', '_', '.'], "-")
                            .to_lowercase()
                            .clone(),
                        config_map,
                    );

                    DagTaskBuilder::default()
                        .name(
                            node.identifier
                                .clone()
                                .replace(['/', ':', '_', '.'], "-")
                                .to_lowercase(),
                        )
                        .template(Some(
                            serde_json::from_value::<String>(node.target["image"].clone())?
                                .replace(['/', ':', '_', '.'], "-"),
                        ))
                        .arguments(Some(
                            ArgumentsBuilder::default()
                                .parameters(vec![{
                                    let mut builder: ParameterBuilder = Default::default();
                                    builder
                                        .name("identifier".to_owned())
                                        .value(Some(
                                            node.identifier
                                                .clone()
                                                .replace(['/', ':', '_', '.'], "-")
                                                .to_lowercase(),
                                        ))
                                        .build()?
                                }])
                                .build()?,
                        ))
                        .build()
                }

                Node::Tabular(node) => {
                    let mut config_map = ConfigMap::default();
                    config_map.metadata.name = Some(
                        node.identifier
                            .replace(['/', ':', '_', '.'], "-")
                            .to_lowercase()
                            .to_owned()
                            + "-config-template",
                    );
                    config_map.data = Some(BTreeMap::from_iter(vec![(
                        "refresh.json".to_owned(),
                        plugin.refresh_config(&node.identifier, &node.branch)?,
                    )]));
                    config_maps.insert(
                        node.identifier
                            .replace(['/', ':', '_', '.'], "-")
                            .to_lowercase()
                            .clone(),
                        config_map,
                    );

                    DagTaskBuilder::default()
                        .name(
                            node.identifier
                                .clone()
                                .replace(['/', ':', '_', '.'], "-")
                                .to_lowercase(),
                        )
                        .template(Some("refresh".to_string()))
                        .arguments(Some(
                            ArgumentsBuilder::default()
                                .parameters(vec![{
                                    let mut builder: ParameterBuilder = Default::default();
                                    builder
                                        .name("identifier".to_owned())
                                        .value(Some(
                                            node.identifier
                                                .clone()
                                                .replace(['/', ':', '_', '.'], "-")
                                                .to_lowercase(),
                                        ))
                                        .build()?
                                }])
                                .build()?,
                        ))
                        .dependencies(
                            dag.dag
                                .neighbors_directed(index, Direction::Incoming)
                                .into_iter()
                                .map(|x| {
                                    dag.dag[x]
                                        .identifier()
                                        .replace(['/', ':', '_', '.'], "-")
                                        .to_lowercase()
                                        .to_owned()
                                })
                                .collect(),
                        )
                        .build()
                }
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
        .kind(Some("CronWorkflow".to_string()))
        .metadata(
            ObjectMetaBuilder::default()
                .name(Some("dashtool".to_owned()))
                .build()?,
        )
        .spec(
            CronWorkflowSpecBuilder::default()
                .schedule("0 0 * * *".to_owned())
                .workflow_spec(
                    WorkflowSpecBuilder::default()
                        .entrypoint(Some("dashtool".to_string()))
                        .templates(templates.into_values().collect())
                        .build()?,
                )
                .build()?,
        )
        .build()?;

    let mut workflow_yaml = serde_yaml::to_string(&workflow)?;

    for config_map in config_maps {
        let yaml = serde_yaml::to_string(&config_map.1)?;
        workflow_yaml.push_str("---\n");
        workflow_yaml.push_str(&yaml);
    }

    fs::write(
        &(WORKFLOW_DIR.to_string() + "/workflow.yaml"),
        workflow_yaml,
    )?;

    Ok(())
}
