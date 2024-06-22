use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    pub fn detach(&mut self) -> Result<()> {
        Err(anyhow!("failed to detach"))
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach()
        .context("Failed to detach the important thing")?;

    let path = &it.path;
    let content =
        fs::read(path).with_context(|| format!("Failed to read instrs from {}", path.display()))?;

    Ok(content)
}

fn main() {
    let it = ImportantThing {
        path: PathBuf::from("important.txt"),
    };

    let content = do_it(it).unwrap();
    println!("{:?}", content);
}
