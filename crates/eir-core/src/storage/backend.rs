use std::path::{Path, PathBuf};

use crate::{
    engine::DatabaseRecord,
    error::{Error, Result},
};

pub struct Backend {
    path: PathBuf,
}

impl Backend {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("database does not exist: {}", path.display()),
            )));
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read(&self) -> Result<DatabaseRecord> {
        let bytes = std::fs::read(&self.path)?;

        rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(&bytes)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    pub fn write(&self, record: &DatabaseRecord) -> Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(record)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        std::fs::write(&self.path, bytes)?;

        Ok(())
    }
}
