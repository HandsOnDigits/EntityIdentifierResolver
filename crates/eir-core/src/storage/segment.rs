use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

const MAGIC: &[u8; 4] = b"EIR\0";
const FORMAT_VERSION: u32 = 1;
const HEADER_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentHeader {
    pub version: u32,
    pub payload_len: u64,
}

impl SegmentHeader {
    pub fn new(payload_len: u64) -> Self {
        Self {
            version: FORMAT_VERSION,
            payload_len,
        }
    }

    fn encode(self) -> [u8; HEADER_SIZE] {
        let mut bytes = [0u8; HEADER_SIZE];

        bytes[0..4].copy_from_slice(MAGIC);
        bytes[4..8].copy_from_slice(&self.version.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.payload_len.to_le_bytes());

        bytes
    }

    fn decode(bytes: &[u8; HEADER_SIZE]) -> Result<Self> {
        if &bytes[0..4] != MAGIC {
            return Err(Error::InvalidFormat("invalid EIR magic".into()));
        }

        let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let payload_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap());

        if version != FORMAT_VERSION {
            return Err(Error::UnsupportedVersion(version));
        }

        Ok(Self {
            version,
            payload_len,
        })
    }
}

pub struct Segment {
    path: PathBuf,
}

impl Segment {
    pub fn create(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "segment does not exist",
            )));
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn write(&self, payload: &[u8]) -> Result<()> {
        let header = SegmentHeader::new(payload.len() as u64);

        let mut file = File::create(&self.path)?;

        file.write_all(&header.encode())?;
        file.write_all(payload)?;
        file.sync_all()?;

        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        let mut file = File::open(&self.path)?;

        let mut header_bytes = [0u8; HEADER_SIZE];
        file.read_exact(&mut header_bytes)?;

        let header = SegmentHeader::decode(&header_bytes)?;

        let payload_len = usize::try_from(header.payload_len)
            .map_err(|_| Error::InvalidFormat("segment payload is too large".into()))?;

        let mut payload = vec![0u8; payload_len];
        file.read_exact(&mut payload)?;

        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_roundtrip() -> Result<()> {
        let path = std::env::temp_dir().join(format!("eir-segment-{}.eir", std::process::id()));

        let segment = Segment::create(&path);

        let payload = b"hello EIR";

        segment.write(payload)?;

        let loaded = segment.read()?;

        assert_eq!(loaded, payload);

        std::fs::remove_file(path).ok();

        Ok(())
    }

    #[test]
    fn segment_rejects_invalid_magic() {
        let path = std::env::temp_dir().join(format!("eir-invalid-{}.eir", std::process::id()));

        let mut bytes = [0u8; HEADER_SIZE];

        bytes[0..4].copy_from_slice(b"BAD!");
        bytes[4..8].copy_from_slice(&FORMAT_VERSION.to_le_bytes());

        std::fs::write(&path, bytes).unwrap();

        let result = Segment::open(&path).unwrap().read();

        assert!(matches!(result, Err(Error::InvalidFormat(_))));

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn segment_rejects_truncated_header() {
        let path = std::env::temp_dir().join(format!("eir-truncated-{}.eir", std::process::id()));

        std::fs::write(&path, b"EIR").unwrap();

        let result = Segment::open(&path).unwrap().read();

        assert!(matches!(result, Err(Error::Io(_))));

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn segment_rejects_unsupported_version() {
        let path = std::env::temp_dir().join(format!("eir-version-{}.eir", std::process::id()));

        let mut bytes = [0u8; HEADER_SIZE];

        bytes[0..4].copy_from_slice(MAGIC);
        bytes[4..8].copy_from_slice(&999u32.to_le_bytes());

        std::fs::write(&path, bytes).unwrap();

        let result = Segment::open(&path).unwrap().read();

        assert!(matches!(result, Err(Error::UnsupportedVersion(999))));

        std::fs::remove_file(path).ok();
    }
}
