mod registry;

use std::fs::File;
use std::io::Seek;

pub fn load_entity<'a>(file_buffer: &'a [u8], entry: EntityIndexEntry) -> &'a ArchivedEntity {
    let start = entry.offset as usize;
    let end = start + entry.size as usize;

    let entity_bytes = &file_buffer[start..end];

    unsafe { rkyv::access_unchecked::<ArchivedEntity>(entity_bytes) }
}

use std::io::Write;

pub fn save_entity(file: &mut File, entity: &Entity) -> Result<u64, EntityError> {
    let offset = file.stream_position()?;

    let bytes = rkyv::to_bytes(entity).map_err(EntityError::Serialize)?;

    file.write_all(&bytes)?;

    Ok(offset)
}
