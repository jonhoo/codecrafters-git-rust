use anyhow::Context;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub(crate) fn invoke(write: bool, file: &Path) -> anyhow::Result<()> {
    fn write_blob<W>(file: &Path, writer: W) -> anyhow::Result<String>
    where
        W: Write,
    {
        let stat = std::fs::metadata(&file).with_context(|| format!("stat {}", file.display()))?;
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter {
            writer,
            hasher: Sha1::new(),
        };
        write!(writer, "blob ")?;
        write!(writer, "{}\0", stat.len())?;
        let mut file =
            std::fs::File::open(&file).with_context(|| format!("open {}", file.display()))?;
        std::io::copy(&mut file, &mut writer).context("stream file into blob")?;
        let _ = writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(hex::encode(hash))
    }

    let hash = if write {
        let tmp = "temporary";
        let hash = write_blob(
            &file,
            std::fs::File::create(tmp).context("construct temporary file for blob")?,
        )
        .context("write out blob object")?;
        fs::create_dir_all(format!(".git/objects/{}/", &hash[..2]))
            .context("create subdir of .git/objects")?;
        std::fs::rename(tmp, format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("move blob file into .git/objects")?;
        hash
    } else {
        write_blob(&file, std::io::sink()).context("write out blob object")?
    };

    println!("{hash}");

    Ok(())
}

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
