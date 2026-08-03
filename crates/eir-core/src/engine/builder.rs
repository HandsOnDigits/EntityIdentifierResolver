use super::Engine;

#[derive(Default)]
pub struct EngineBuilder {
    path: Option<String>,
}

impl EngineBuilder {
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(self) -> crate::error::Result<Engine> {
        let path = self.path.unwrap_or_else(|| "eir.db".into());

        Engine::open(&path)
    }
}
