use crate::objects::{Kind, Object};
use anyhow::Context;
use std::fmt::Write;
use std::io::Cursor;

pub(crate) fn invoke(
    message: String,
    tree_hash: String,
    parent_hash: Option<String>,
) -> anyhow::Result<()> {
    // NOTE: the ?s here for write will never trigger as we're writing into a String.
    let mut commit = String::new();
    writeln!(commit, "tree {tree_hash}")?;
    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {parent_hash}")?;
    }
    writeln!(
        commit,
        "author Jon Gjengset <jon@thesquareplanet.com> 1709990458 +0100"
    )?;
    writeln!(
        commit,
        "committer Jon Gjengset <jon@thesquareplanet.com> 1709990458 +0100"
    )?;
    writeln!(commit, "")?;
    writeln!(commit, "{message}")?;
    let hash = Object {
        kind: Kind::Commit,
        expected_size: commit.len() as u64,
        reader: Cursor::new(commit),
    }
    .write_to_objects()
    .context("write commit object")?;

    println!("{}", hex::encode(hash));

    Ok(())
}
