use anyhow::anyhow;
use git2::{Diff, Oid, Repository};

use crate::error::Error;

pub(crate) fn diff<'a>(
    repo: &'a Repository,
    old_id: &Option<Oid>,
    new_id: &Option<Oid>,
) -> Result<Diff<'a>, Error> {
    let previous_commit = old_id
        .as_ref()
        .cloned()
        .map(|x| repo.find_commit(x))
        .transpose()?
        .map(|x| x.tree())
        .transpose()?;

    let current_commit = new_id
        .as_ref()
        .and_then(|object| repo.find_commit(object.clone()).ok())
        .and_then(|commit| commit.tree().ok());

    let diff = repo.diff_tree_to_tree(previous_commit.as_ref(), current_commit.as_ref(), None)?;

    Ok(diff)
}

pub(crate) fn branch(repo: &Repository) -> Result<String, Error> {
    Ok(repo
        .find_reference("HEAD")?
        .symbolic_target()
        .ok_or(Error::Anyhow(anyhow!(
            "Dashtool cannot run with uncommited changes. Please commit all your changes."
        )))?
        .trim_start_matches("refs/heads/")
        .to_owned())
}
