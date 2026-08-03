use crate::{error::Result, storage::store::Store};

pub struct Backend {
    store: Store,
}

impl Backend {
    pub fn create() -> Store {
        Store::new()
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let store = Store::load(path)?;

        Ok(Self { store })
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn flush(&self) -> Result<()> {
        self.store.save()
    }
}
