use anyhow::Result;
use rkyv::{Archive, Serialize, to_bytes};
use std::fs;

pub fn write_binary<T>(path: &str, value: &T) -> Result<()>
where
    T: Archive,
    T: for<'a> Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error,
        >,
    >,
{
    let bytes = to_bytes::<rkyv::rancor::Error>(value)?;

    fs::write(path, &bytes)?;

    Ok(())
}
