use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SyncConfig {
    pub storage_url: Option<String>,
    pub token: Option<String>,
    pub last_sync: Option<String>,
    pub auto_sync: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmbedConfig {
    pub api_url: Option<String>,
    pub token: Option<String>,
}
