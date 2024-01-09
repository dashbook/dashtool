use std::{fs, sync::Arc};

use git2::{BranchType, Repository};

use crate::{
    error::Error,
    git::{branch, diff},
    plugins::Plugin,
    state::State,
};

use self::{build_dag::build_dag, update_dag::update_dag};

mod build_dag;
mod update_dag;

pub async fn build(plugin: Arc<dyn Plugin>) -> Result<(), Error> {
    let mut state: State = fs::read_to_string(".dashtool/state.json")
        .ok()
        .and_then(|x| serde_json::from_str(&x).ok())
        .unwrap_or_default();

    // Inspect git repo
    let repo = Repository::open(".")?;

    // Name of the currently selected branch in the git repo
    let current_branch = branch(&repo)?;

    let current_id = repo
        .find_branch(&current_branch, BranchType::Local)?
        .into_reference()
        .target();

    let main_id = repo
        .find_branch("main", BranchType::Local)?
        .into_reference()
        .target();

    // Id of the last commit on the current brranch that was built with dashtool
    let last_id = state.commits.get(&current_branch).cloned();

    // Id of the last commit on the main branch that was built with dashtool
    let last_main_id = state.commits.get("main").cloned();

    // Check if dashtool built a branch with the same commit as the current main branch to see if the branch was merged
    let merged_branch = state
        .commits
        .iter()
        .find(|(k, v)| {
            if let Some(y) = &main_id {
                *v == y && *k != "main"
            } else {
                false
            }
        })
        .map(|(k, _)| k)
        .cloned();

    let last_main_diff = diff(&repo, &None, &last_main_id)?;

    let mut dag = update_dag(last_main_diff, None, "main")?;

    let main_diff = diff(&repo, &last_main_id, &main_id)?;

    build_dag(
        &mut dag,
        main_diff,
        plugin.clone(),
        "main",
        merged_branch.as_deref(),
    )
    .await?;

    let last_diff = diff(&repo, &main_id, &last_id)?;

    let mut dag = update_dag(last_diff, Some(dag), &current_branch)?;

    let diff = diff(&repo, &last_id.or(main_id), &current_id)?;

    build_dag(&mut dag, diff, plugin.clone(), &current_branch, None).await?;

    let json = serde_json::to_string(&dag)?;

    fs::write(
        ".dashtool/dags/".to_string() + &current_branch + ".json",
        json,
    )?;

    if let Some(current_id) = current_id {
        state.commits.insert(current_branch, current_id);
    }

    if let Some(main_id) = main_id {
        state.commits.insert("main".to_owned(), main_id);
    }

    let state_json = serde_json::to_string(&state)?;

    fs::write(".dashtool/state.json", state_json)?;

    Ok(())
}
