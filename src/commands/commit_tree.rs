use crate::objects::{Kind, Object};
use anyhow::Context;
use std::env;
use std::fmt::Write;
use std::io::Cursor;

pub(crate) fn write_commit(
    message: &str,
    tree_hash: &str,
    parent_hash: Option<&str>,
) -> anyhow::Result<[u8; 20]> {
    // NOTE: the ?s here for write will never trigger as we're writing into a String.
    let mut commit = String::new();
    writeln!(commit, "tree {tree_hash}")?;
    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {parent_hash}")?;
    }
    let (name, email) =
        if let (Some(name), Some(email)) = (env::var_os("NAME"), env::var_os("EMAIL")) {
            let name = name
                .into_string()
                .map_err(|_| anyhow::anyhow!("$NAME is invalid utf-8"))?;
            let email = email
                .into_string()
                .map_err(|_| anyhow::anyhow!("$EMAIL is invalid utf-8"))?;
            (name, email)
        } else {
            (
                String::from("Jon Gjengset"),
                String::from("jon@thesquareplanet.com"),
            )
        };
    let time = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .context("current system time is before UNIX epoch")?;
    writeln!(commit, "author {name} <{email}> {} +0000", time.as_secs())?;
    writeln!(
        commit,
        "committer {name} <{email}> {} +0000",
        time.as_secs()
    )?;
    writeln!(commit, "")?;
    writeln!(commit, "{message}")?;
    Object {
        kind: Kind::Commit,
        expected_size: commit.len() as u64,
        reader: Cursor::new(commit),
    }
    .write_to_objects()
    .context("write commit object")
}

pub(crate) fn invoke(
    message: String,
    tree_hash: String,
    parent_hash: Option<String>,
) -> anyhow::Result<()> {
    let hash =
        write_commit(&message, &tree_hash, parent_hash.as_deref()).context("create commit")?;

    println!("{}", hex::encode(hash));

    Ok(())
}
