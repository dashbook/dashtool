use git2::{Diff, Oid, Repository};

use crate::error::Error;

pub(crate) fn diff<'a>(repo: &'a Repository, last_commit: &Option<Oid>) -> Result<Diff<'a>, Error> {
    let previous_id = last_commit.as_ref();

    let current_object = repo.revparse_single("HEAD")?;

    let current_id = current_object.id();

    let previous_commit = previous_id
        .cloned()
        .map(|x| repo.find_commit(x))
        .transpose()?
        .map(|x| x.tree())
        .transpose()?;

    let current_commit = repo.find_commit(current_id)?.tree()?;

    let diff = repo.diff_tree_to_tree(previous_commit.as_ref(), Some(&current_commit), None)?;

    Ok(diff)
}
