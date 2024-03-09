use anyhow::Context;

pub(crate) fn invoke(name_only: bool) -> anyhow::Result<()> {
    anyhow::ensure!(name_only, "only --name-only is supported for now");

    todo!();

    Ok(())
}
