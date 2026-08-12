use std::sync::Arc;

use eir_core::engine::Engine;

pub struct AppState {
    pub engine: Arc<Engine>,
    pub firewall: Firewall,
}
