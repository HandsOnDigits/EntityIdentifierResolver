pub mod debug;
pub mod generate;
pub mod index;
//pub mod search;

use eir_core::{error::Result, storage::Store};

pub struct Engine {
    store: Store,
}

impl Engine {
    pub fn open(path: &str) -> Result<Self> {
        let store = Store::load(path)?;

        Ok(Self { store })
    }
}
