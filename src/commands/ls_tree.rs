use crate::objects::{Kind, Object};
use anyhow::Context;
use std::{
    ffi::CStr,
    io::{BufRead, Read, Write},
};

pub(crate) fn invoke(name_only: bool, tree_hash: &str) -> anyhow::Result<()> {
    let mut object = Object::read(tree_hash).context("parse out tree object file")?;
    match object.kind {
        Kind::Tree => {
            let mut buf = Vec::new();
            let mut hashbuf = [0; 20];
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            loop {
                buf.clear();
                let n = object
                    .reader
                    .read_until(0, &mut buf)
                    .context("read next tree object entry")?;
                if n == 0 {
                    break;
                }
                object
                    .reader
                    .read_exact(&mut hashbuf[..])
                    .context("read tree entry object hash")?;

                let mode_and_name =
                    CStr::from_bytes_with_nul(&buf).context("invalid tree entry")?;
                // TODO: replace with split_once: https://github.com/rust-lang/rust/issues/112811
                let mut bits = mode_and_name.to_bytes().splitn(2, |&b| b == b' ');
                let mode = bits.next().expect("split always yields once");
                let name = bits
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("tree entry has no file name"))?;

                if name_only {
                    stdout
                        .write_all(name)
                        .context("write tree entry name to stdout")?;
                } else {
                    let mode = std::str::from_utf8(mode).context("mode is always valid utf-8")?;
                    let hash = hex::encode(&hashbuf);
                    let object = Object::read(&hash)
                        .with_context(|| format!("read object for tree entry {hash}"))?;
                    write!(stdout, "{mode:0>6} {} {hash} ", object.kind)
                        .context("write tree entry meta to stdout")?;
                    stdout
                        .write_all(name)
                        .context("write tree entry name to stdout")?;
                }
                writeln!(stdout, "").context("write newline to stdout")?;
            }
        }
        _ => anyhow::bail!("don't yet know how to ls '{}'", object.kind),
    }

    Ok(())
}
