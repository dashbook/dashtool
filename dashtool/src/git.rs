use anyhow::anyhow;
use gix::{
    diff::tree::{recorder::Change, Changes, Recorder},
    prelude::Find,
    ObjectId, OdbHandle, Repository,
};

use crate::error::Error;

pub(crate) fn diff<'a>(
    db: &OdbHandle,
    old_id: &Option<ObjectId>,
    new_id: &Option<ObjectId>,
) -> Result<Vec<Change>, Error> {
    let mut old_commit_buffer = Vec::new();
    let mut new_commit_buffer = Vec::new();
    let mut old_tree_buffer = Vec::new();
    let mut new_tree_buffer = Vec::new();
    let old_tree = old_id
        .as_ref()
        .and_then(|x| db.try_find(&x, &mut old_commit_buffer).transpose())
        .transpose()?
        .and_then(|x| x.try_into_commit_iter())
        .map(|mut x| x.tree_id())
        .transpose()?
        .and_then(|x| db.try_find(&x, &mut old_tree_buffer).transpose())
        .transpose()?
        .and_then(|x| x.try_into_tree_iter());

    let new_tree = new_id
        .as_ref()
        .or(old_id.as_ref())
        .and_then(|x| db.try_find(&x, &mut new_commit_buffer).transpose())
        .transpose()?
        .and_then(|x| x.try_into_commit_iter())
        .map(|mut x| x.tree_id())
        .transpose()?
        .and_then(|x| db.try_find(&x, &mut new_tree_buffer).transpose())
        .transpose()?
        .and_then(|x| x.try_into_tree_iter());

    let mut recorder = Recorder::default();

    if let Some(new_tree) = new_tree {
        Changes::from(old_tree).needed_to_obtain(
            new_tree,
            gix::diff::tree::State::default(),
            db,
            &mut recorder,
        )?;
    }

    let diff = recorder.records;

    Ok(diff)
}

pub(crate) fn branch(repo: &Repository) -> Result<String, Error> {
    String::from_utf8(
        repo.find_reference("HEAD")?
            .target()
            .try_name()
            .ok_or(Error::Anyhow(anyhow!(
                "Dashtool cannot run with uncommited changes. Please commit all your changes."
            )))?
            .as_bstr()
            .strip_prefix("refs/heads/".as_bytes())
            .to_owned()
            .ok_or(Error::Anyhow(anyhow!(
                "Dashtool cannot run with uncommited changes. Please commit all your changes."
            )))?
            .to_owned(),
    )
    .map_err(Error::from)
}
