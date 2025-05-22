use crate::config::models::{EmbedConfig, SyncConfig};
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

const ENCRYPTION_SALT: &[u8] = b"GNEDBY_TOKEN_ENCRYPTION_SALT";

fn get_config_path(config_name: &str) -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;
    let config_dir = project_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join(format!("{}.json", config_name)))
}

fn get_encryption_key() -> Result<Vec<u8>> {
    let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;
    let config_path = project_dirs.config_dir().to_string_lossy().to_string();
    let mut hasher = Sha256::new();
    hasher.update(config_path.as_bytes());
    hasher.update(ENCRYPTION_SALT);
    Ok(hasher.finalize().to_vec())
}

fn encrypt_token(token: &str) -> Result<String> {
    let key = get_encryption_key()?;
    let token_bytes = token.as_bytes();
    let mut encrypted = Vec::with_capacity(token_bytes.len());
    for (i, &byte) in token_bytes.iter().enumerate() {
        encrypted.push(byte ^ key[i % key.len()]);
    }
    Ok(general_purpose::STANDARD.encode(encrypted))
}

fn decrypt_token(encrypted: &str) -> Result<String> {
    let key = get_encryption_key()?;
    let encrypted_bytes = general_purpose::STANDARD
        .decode(encrypted)
        .context("Failed to decode encrypted token")?;
    let mut decrypted = Vec::with_capacity(encrypted_bytes.len());
    for (i, &byte) in encrypted_bytes.iter().enumerate() {
        decrypted.push(byte ^ key[i % key.len()]);
    }
    String::from_utf8(decrypted).context("Failed to decode token to valid UTF-8")
}

pub trait TokenConfig {
    fn get_token(&self) -> Option<&str>;
    fn set_token(&mut self, token: String);
}

impl TokenConfig for SyncConfig {
    fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

impl TokenConfig for EmbedConfig {
    fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

fn load_config<T: DeserializeOwned + Default + TokenConfig>(config_name: &str) -> Result<T> {
    let config_path = get_config_path(config_name)?;
    if !config_path.exists() {
        return Ok(T::default());
    }
    let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let mut config: T = serde_json::from_str(&config_str).context("Failed to parse config file")?;
    if let Some(stored_token) = config.get_token() {
        if !stored_token.is_empty() {
            match decrypt_token(stored_token) {
                Ok(decrypted) => config.set_token(decrypted),
                Err(_) => println!("Note: Will encrypt token on next save"),
            }
        }
    }
    Ok(config)
}

fn save_config<T: Serialize + Clone + TokenConfig>(config: &T, config_name: &str) -> Result<()> {
    let mut config_to_save = config.clone();
    if let Some(token) = config.get_token() {
        if !token.is_empty() {
            match encrypt_token(token) {
                Ok(encrypted) => config_to_save.set_token(encrypted),
                Err(e) => eprintln!("Warning: could not encrypt token: {}", e),
            }
        }
    }
    let config_path = get_config_path(config_name)?;
    let config_str =
        serde_json::to_string_pretty(&config_to_save).context("Failed to serialize config")?;
    fs::write(&config_path, config_str).context("Failed to write config file")?;
    Ok(())
}

// Public API for each config type
pub fn load_sync_config() -> Result<SyncConfig> {
    load_config("sync_config")
}
pub fn save_sync_config(config: &SyncConfig) -> Result<()> {
    save_config(config, "sync_config")
}
pub fn load_embed_config() -> Result<EmbedConfig> {
    load_config("embed_config")
}
pub fn save_embed_config(config: &EmbedConfig) -> Result<()> {
    save_config(config, "embed_config")
}
