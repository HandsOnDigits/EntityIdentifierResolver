use anyhow::Result;
use rkyv::to_bytes;
use std::fs;

pub fn write_binary<T>(path: &str, value: &T) -> Result<()>
where
    T: rkyv::Archive + rkyv::Serialize<rkyv::rancor::Error>,
{
    let bytes = to_bytes::<rkyv::rancor::Error>(value)?;

    fs::write(path, bytes)?;

    Ok(())
}
