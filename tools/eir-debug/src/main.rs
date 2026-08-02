mod read;

use anyhow::Result;

fn main() -> Result<()> {
    read::read_entities("output/entities.bin")?;

    Ok(())
}
