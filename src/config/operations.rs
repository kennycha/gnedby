use crate::config::models::SyncConfig;
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use directories::ProjectDirs;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

const ENCRYPTION_SALT: &[u8] = b"GNEDBY_TOKEN_ENCRYPTION_SALT";

fn get_config_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;

    let config_dir = project_dirs.config_dir();
    fs::create_dir_all(config_dir)?;

    let config_path = config_dir.join("sync_config.json");
    Ok(config_path)
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

pub fn load_config() -> Result<SyncConfig> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(SyncConfig::default());
    }

    let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let mut config: SyncConfig =
        serde_json::from_str(&config_str).context("Failed to parse config file")?;

    if let Some(ref stored_token) = config.token {
        if !stored_token.is_empty() {
            match decrypt_token(stored_token) {
                Ok(decrypted) => {
                    config.token = Some(decrypted);
                }
                Err(_) => {
                    println!("Note: Will encrypt token on next save");
                }
            }
        }
    }

    Ok(config)
}

pub fn save_config(config: &SyncConfig) -> Result<()> {
    let mut config_to_save = config.clone();

    if let Some(token) = &config.token {
        if !token.is_empty() {
            match encrypt_token(token) {
                Ok(encrypted) => config_to_save.token = Some(encrypted),
                Err(e) => {
                    eprintln!("Warning: could not encrypt token: {}", e);
                }
            }
        }
    }

    let config_path = get_config_path()?;
    let config_str =
        serde_json::to_string_pretty(&config_to_save).context("Failed to serialize config")?;

    fs::write(&config_path, config_str).context("Failed to write config file")?;

    Ok(())
}
