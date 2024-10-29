use std::sync::Arc;

use tracing::info;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct Services {
    pub config: Arc<AppConfig>,
}

impl Services {
    pub fn new(config: Arc<AppConfig>) -> Self {
        info!("initializing services...");
        Self { config }
    }
}
