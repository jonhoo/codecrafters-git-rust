use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) mod commands;
pub(crate) mod objects;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum Command {
    /// Doc comment
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: PathBuf,
    },
    LsTree {
        #[clap(long)]
        name_only: bool,

        tree_hash: String,
    },
    WriteTree,
    CommitTree {
        #[clap(short = 'm')]
        message: String,

        #[clap(short = 'p')]
        parent_hash: Option<String>,

        tree_hash: String,
    },
    Commit {
        #[clap(short = 'm')]
        message: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => commands::cat_file::invoke(pretty_print, &object_hash)?,
        Command::HashObject { write, file } => commands::hash_object::invoke(write, &file)?,
        Command::LsTree {
            name_only,
            tree_hash,
        } => commands::ls_tree::invoke(name_only, &tree_hash)?,
        Command::WriteTree => commands::write_tree::invoke()?,
        Command::CommitTree {
            message,
            tree_hash,
            parent_hash,
        } => commands::commit_tree::invoke(message, tree_hash, parent_hash)?,
        Command::Commit { message } => {
            let head_ref = std::fs::read_to_string(".git/HEAD").context("read HEAD")?;
            let Some(head_ref) = head_ref.strip_prefix("ref: ") else {
                anyhow::bail!("refusing to commit onto detached HEAD");
            };
            let head_ref = head_ref.trim();
            let parent_hash = std::fs::read_to_string(format!(".git/{head_ref}"))
                .with_context(|| format!("read HEAD reference target '{head_ref}'"))?;
            let parent_hash = parent_hash.trim();

            let Some(tree_hash) =
                commands::write_tree::write_tree_for(Path::new(".")).context("write tree")?
            else {
                eprintln!("not committing empty tree");
                return Ok(());
            };

            let commit_hash = commands::commit_tree::write_commit(
                &message,
                &hex::encode(tree_hash),
                Some(parent_hash),
            )
            .context("create commit")?;
            let commit_hash = hex::encode(commit_hash);

            std::fs::write(format!(".git/{head_ref}"), &commit_hash)
                .with_context(|| format!("update HEAD reference target {head_ref}"))?;

            println!("HEAD is now at {commit_hash}");
        }
    }

    Ok(())
}
