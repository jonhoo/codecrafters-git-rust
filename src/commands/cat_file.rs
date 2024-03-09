use crate::objects::{Kind, Object};
use anyhow::Context;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        pretty_print,
        "mode must be given without -p, and we don't support mode"
    );

    let mut object = Object::read(object_hash).context("parse out blob object file")?;
    match object.kind {
        Kind::Blob => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n = std::io::copy(&mut object.reader, &mut stdout)
                .context("write .git/objects file to stdout")?;
            anyhow::ensure!(
                n == object.expected_size,
                ".git/object file was not the expected size (expected: {}, actual: {n})",
                object.expected_size
            );
        }
        _ => anyhow::bail!("don't yet know how to print '{}'", object.kind),
    }

    Ok(())
}
