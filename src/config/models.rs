use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncConfig {
    pub storage_url: Option<String>,
    pub token: Option<String>,
    pub last_sync: Option<String>,
    pub auto_sync: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            storage_url: None,
            token: None,
            last_sync: None,
            auto_sync: false,
        }
    }
}
