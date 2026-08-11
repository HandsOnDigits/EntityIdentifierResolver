use crate::{
    config::StorageConfig,
    engine::DatabaseRecord,
    error::{Error, Result},
};

use super::{
    segment_manager::SegmentManager,
    wal::{Wal, WalOperation},
};

pub struct Backend {
    segments: SegmentManager,
    wal: Wal,
}

impl Backend {
    pub fn create(config: StorageConfig) -> Result<Self> {
        let wal_path = config.wal_path();

        let segments = SegmentManager::create(config.clone())?;
        let wal = Wal::create(wal_path)?;

        Ok(Self { segments, wal })
    }

    pub fn open(config: StorageConfig) -> Result<Self> {
        let segments = match SegmentManager::open(config.clone()) {
            Ok(segments) => segments,
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                SegmentManager::create(config.clone())?
            }
            Err(error) => return Err(error),
        };

        let wal = match Wal::open(config.wal_path()) {
            Ok(wal) => wal,
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                Wal::create(config.wal_path())?
            }
            Err(error) => return Err(error),
        };

        Ok(Self { segments, wal })
    }

    pub fn append(&mut self, operation: &WalOperation) -> Result<()> {
        self.wal.append(operation)
    }

    pub fn replay(&self) -> Result<Vec<WalOperation>> {
        self.wal.replay()
    }

    pub fn write(&mut self, record: &DatabaseRecord) -> Result<()> {
        self.segments.write_record(record)?;
        self.wal.truncate()?;

        Ok(())
    }

    pub fn read(&self) -> Result<DatabaseRecord> {
        self.segments.read_record()
    }

    pub fn wal(&self) -> &Wal {
        &self.wal
    }

    pub fn compact(&mut self, record: &DatabaseRecord) -> Result<()> {
        self.segments.rewrite(record)?;
        self.wal.truncate()?;
        Ok(())
    }
}
