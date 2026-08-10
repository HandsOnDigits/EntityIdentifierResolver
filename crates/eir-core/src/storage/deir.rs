use crate::error::{Error, Result};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

const MAGIC: &[u8; 4] = b"DEIR";
const FORMAT_VERSION: u16 = 1;
const HEADER_SIZE: usize = 16;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeirKind {
    Segment = 1,
    Wal = 2,
}

impl DeirKind {
    fn from_u8(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Segment),
            2 => Ok(Self::Wal),
            _ => Err(Error::InvalidFormat(format!(
                "unknown DEIR file kind: {value}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeirHeader {
    pub version: u16,
    pub kind: DeirKind,
    pub flags: u8,
    pub payload_len: u64,
}

impl DeirHeader {
    pub fn new(kind: DeirKind, payload_len: u64) -> Self {
        Self {
            version: FORMAT_VERSION,
            kind,
            flags: 0,
            payload_len,
        }
    }

    fn encode(self) -> [u8; HEADER_SIZE] {
        let mut bytes = [0u8; HEADER_SIZE];

        bytes[0..4].copy_from_slice(MAGIC);
        bytes[4..6].copy_from_slice(&self.version.to_le_bytes());
        bytes[6] = self.kind as u8;
        bytes[7] = self.flags;
        bytes[8..16].copy_from_slice(&self.payload_len.to_le_bytes());

        bytes
    }

    fn decode(bytes: &[u8; HEADER_SIZE]) -> Result<Self> {
        if &bytes[0..4] != MAGIC {
            return Err(Error::InvalidFormat("invalid DEIR magic".into()));
        }

        let version = u16::from_le_bytes(bytes[4..6].try_into().unwrap());

        if version != FORMAT_VERSION {
            return Err(Error::UnsupportedVersion(version as u32));
        }

        let kind = DeirKind::from_u8(bytes[6])?;
        let flags = bytes[7];

        let payload_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap());

        Ok(Self {
            version,
            kind,
            flags,
            payload_len,
        })
    }
}

pub struct DeirFile {
    path: PathBuf,
    kind: DeirKind,
}

impl DeirFile {
    pub fn create(path: impl AsRef<Path>, kind: DeirKind) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(&path)?;

        let header = DeirHeader::new(kind, 0);

        file.write_all(&header.encode())?;
        file.sync_all()?;

        Ok(Self { path, kind })
    }

    pub fn open(path: impl AsRef<Path>, kind: DeirKind) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DEIR file does not exist",
            )));
        }

        // Validate the existing file immediately.
        let file = File::open(&path)?;

        let mut header_bytes = [0u8; HEADER_SIZE];
        let mut reader = std::io::BufReader::new(file);

        reader.read_exact(&mut header_bytes)?;

        let header = DeirHeader::decode(&header_bytes)?;

        if header.kind != kind {
            return Err(Error::InvalidFormat(format!(
                "expected {:?} file, found {:?}",
                kind, header.kind
            )));
        }

        Ok(Self { path, kind })
    }

    pub fn write(&self, payload: &[u8]) -> Result<()> {
        let payload_len = u64::try_from(payload.len())
            .map_err(|_| Error::InvalidFormat("DEIR payload is too large".into()))?;

        let header = DeirHeader::new(self.kind, payload_len);

        let mut file = File::create(&self.path)?;

        file.write_all(&header.encode())?;
        file.write_all(payload)?;
        file.set_len(HEADER_SIZE as u64 + payload_len)?;
        file.sync_all()?;

        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        let mut file = File::open(&self.path)?;

        if file.metadata()?.len() == 0 {
            return Ok(Vec::new());
        }

        let mut header_bytes = [0u8; HEADER_SIZE];
        file.read_exact(&mut header_bytes)?;

        let header = DeirHeader::decode(&header_bytes)?;

        if header.kind != self.kind {
            return Err(Error::InvalidFormat(format!(
                "expected {:?} file, found {:?}",
                self.kind, header.kind
            )));
        }

        let payload_len = usize::try_from(header.payload_len)
            .map_err(|_| Error::InvalidFormat("DEIR payload is too large".into()))?;

        let mut payload = vec![0u8; payload_len];

        file.read_exact(&mut payload)?;

        Ok(payload)
    }

    pub fn truncate(&self) -> Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;

        file.set_len(HEADER_SIZE as u64)?;
        file.seek(std::io::SeekFrom::Start(0))?;

        let header = DeirHeader::new(self.kind, 0);

        file.write_all(&header.encode())?;
        file.flush()?;
        file.sync_all()?;

        Ok(())
    }

    pub fn sync(&self) -> Result<()> {
        let file = OpenOptions::new().read(true).write(true).open(&self.path)?;

        file.sync_all()?;

        Ok(())
    }

    pub fn size(&self) -> Result<u64> {
        match std::fs::metadata(&self.path) {
            Ok(metadata) => Ok(metadata.len()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(error) => Err(error.into()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(&self, payload: &[u8]) -> Result<()> {
        let current_len = self.payload_len()?;

        let payload_len = u64::try_from(payload.len())
            .map_err(|_| Error::InvalidFormat("DEIR payload is too large".into()))?;

        let new_len = current_len
            .checked_add(payload_len)
            .ok_or_else(|| Error::InvalidFormat("DEIR payload is too large".into()))?;

        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;

        // Append after the existing payload.
        file.seek(std::io::SeekFrom::Start(HEADER_SIZE as u64 + current_len))?;

        file.write_all(payload)?;

        // Update only the header's payload length.
        file.seek(std::io::SeekFrom::Start(0))?;

        let header = DeirHeader::new(self.kind, new_len);
        file.write_all(&header.encode())?;

        file.flush()?;
        file.sync_all()?;

        Ok(())
    }

    fn read_header(&self) -> Result<DeirHeader> {
        let mut file = File::open(&self.path)?;

        let mut bytes = [0u8; HEADER_SIZE];
        file.read_exact(&mut bytes)?;

        let header = DeirHeader::decode(&bytes)?;

        if header.kind != self.kind {
            return Err(Error::InvalidFormat(format!(
                "expected {:?} file, found {:?}",
                self.kind, header.kind
            )));
        }

        Ok(header)
    }

    fn payload_len(&self) -> Result<u64> {
        Ok(self.read_header()?.payload_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deir_append_roundtrip() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("test.deir");

        let file = DeirFile::create(&path, DeirKind::Wal)?;

        file.append(b"hello")?;
        file.append(b" world")?;

        assert_eq!(file.read()?, b"hello world");

        Ok(())
    }

    #[test]
    fn deir_truncate_preserves_header() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("test.deir");

        let file = DeirFile::create(&path, DeirKind::Wal)?;

        file.append(b"hello")?;
        file.truncate()?;

        assert!(file.read()?.is_empty());

        let reopened = DeirFile::open(&path, DeirKind::Wal)?;
        assert!(reopened.read()?.is_empty());

        Ok(())
    }
}
