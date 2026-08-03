mod builder;

use std::path::Path;

use crate::{error::Result, storage::Backend};

pub struct Engine {
    backend: Backend,
}

impl Engine {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            backend: Backend::open(path)?,
        })
    }
}
