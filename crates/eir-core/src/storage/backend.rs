use crate::{config::StorageConfig, engine::DatabaseRecord, error::Result};

use super::segment_manager::SegmentManager;

pub struct Backend {
    segments: SegmentManager,
}

impl Backend {
    pub fn create(config: StorageConfig) -> Result<Self> {
        let segments = SegmentManager::create(config)?;

        Ok(Self { segments })
    }

    pub fn open(config: StorageConfig) -> Result<Self> {
        let segments = SegmentManager::open(config)?;

        Ok(Self { segments })
    }

    pub fn write(&mut self, record: &DatabaseRecord) -> Result<()> {
        self.segments.write_record(record)
    }

    pub fn read(&self) -> Result<DatabaseRecord> {
        self.segments.read_record()
    }
}
