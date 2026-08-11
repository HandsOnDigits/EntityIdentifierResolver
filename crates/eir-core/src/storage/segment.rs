use std::path::Path;

use crate::{
    error::Result,
    storage::deir::{DeirFile, DeirKind},
};

pub struct Segment {
    file: DeirFile,
}

impl Segment {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: DeirFile::create(path, DeirKind::Segment)?,
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: DeirFile::open(path, DeirKind::Segment)?,
        })
    }

    pub fn path(&self) -> &Path {
        self.file.path()
    }

    pub fn write(&self, payload: &[u8]) -> Result<()> {
        self.file.write(payload)
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        self.file.read()
    }

    pub fn size(&self) -> Result<u64> {
        self.file.size()
    }

    pub fn into_file(self) -> DeirFile {
        self.file
    }
}
