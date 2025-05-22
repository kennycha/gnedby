use crate::config::{load_sync_config, save_sync_config};
use crate::db::calculate_db_hash;
use crate::db::get_db_path;
use crate::db::models::Album;
use crate::db::operations::Database;
use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct SyncMetadata {
    hash: String,
    last_sync: String,
}

async fn get_remote_database(storage_url: &str, token: &str) -> Result<Vec<Album>> {
    let db_url = format!("{}/albums.db", storage_url);
    let client = create_client(token)?;

    let response = client
        .get(&db_url)
        .send()
        .await
        .context("Failed to download remote database")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download database: {}",
            response.status()
        ));
    }

    let db_content = response
        .bytes()
        .await
        .context("Failed to read database content")?;

    let db_path = get_db_path()?;
    let backup_path = db_path.with_extension("db.temp");
    fs::write(&backup_path, &db_content).context("Failed to write temporary database file")?;

    let remote_db = Database::with_path(&backup_path).await?;
    let remote_albums = remote_db.get_all_albums().await?;

    fs::remove_file(&backup_path).ok();
    Ok(remote_albums)
}

pub async fn check_sync_status(verbose: bool) -> Result<bool> {
    let config = load_sync_config()?;

    if config.storage_url.is_none() || config.token.is_none() {
        println!("Sync is not configured. Please set storage_url and token first:");
        println!("  gnedby sync config set storage_url <url>");
        println!("  gnedby sync config set token <token>");
        return Ok(false);
    }

    let storage_url = config.storage_url.unwrap();
    let token = config.token.unwrap();

    let local_hash = calculate_db_hash()?;
    let remote_metadata = get_remote_metadata(&storage_url, &token).await?;

    if local_hash == remote_metadata.hash {
        println!("Your database is up to date!");
        println!("Last sync: {}", remote_metadata.last_sync);
        Ok(true)
    } else {
        println!("Your database is out of sync with remote.");
        println!("Last sync: {}", remote_metadata.last_sync);

        if verbose {
            let local_db = Database::new().await?;
            let local_albums = local_db.get_all_albums().await?;
            let local_map: HashMap<i64, Album> = local_albums
                .into_iter()
                .map(|a| (a.id.unwrap(), a))
                .collect();

            let remote_albums = get_remote_database(&storage_url, &token).await?;
            let remote_map: HashMap<i64, Album> = remote_albums
                .into_iter()
                .map(|a| (a.id.unwrap(), a))
                .collect();

            let mut added = 0;
            let mut deleted = 0;
            let mut updated = 0;

            for (id, local_album) in &local_map {
                match remote_map.get(id) {
                    Some(remote_album) if remote_album != local_album => {
                        updated += 1;
                    }
                    None => {
                        deleted += 1;
                    }
                    _ => {}
                }
            }

            for id in remote_map.keys() {
                if !local_map.contains_key(id) {
                    added += 1;
                }
            }

            println!("\nChanges:");
            println!("  Added:   {} album(s)", added);
            println!("  Deleted: {} album(s)", deleted);
            println!("  Updated: {} album(s)", updated);
        }

        Ok(false)
    }
}

pub async fn pull_from_remote() -> Result<()> {
    let config = load_sync_config()?;

    if config.storage_url.is_none() || config.token.is_none() {
        println!("Sync is not configured. Please set storage_url and token first.");
        return Ok(());
    }

    let storage_url = config.storage_url.unwrap();
    let token = config.token.unwrap();

    backup_database()?;

    let db_url = format!("{}/albums.db", storage_url);
    let client = create_client(&token)?;

    let response = client
        .get(&db_url)
        .send()
        .await
        .context("Failed to download remote database")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download database: {}",
            response.status()
        ));
    }

    let db_content = response
        .bytes()
        .await
        .context("Failed to read database content")?;

    let db_path = get_db_path()?;
    fs::write(&db_path, &db_content).context("Failed to write database file")?;

    println!("Database pulled successfully!");

    let local_hash = calculate_db_hash()?;
    let metadata = SyncMetadata {
        hash: local_hash,
        last_sync: Utc::now().to_rfc3339(),
    };

    let mut config = load_sync_config()?;
    config.last_sync = Some(metadata.last_sync.clone());
    save_sync_config(&config)?;

    Ok(())
}

pub async fn push_to_remote() -> Result<()> {
    let config = load_sync_config()?;

    if config.storage_url.is_none() || config.token.is_none() {
        println!("Sync is not configured. Please set storage_url and token first.");
        return Ok(());
    }

    let storage_url = config.storage_url.unwrap();
    let token = config.token.unwrap();

    let db_path = get_db_path()?;
    let db_content = fs::read(&db_path).context("Failed to read local database file")?;

    let db_url = format!("{}/albums.db", storage_url);
    let client = create_client(&token)?;

    let response = client
        .put(&db_url)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(db_content)
        .send()
        .await
        .context("Failed to upload database")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to upload database: {}",
            response.status()
        ));
    }

    let local_hash = calculate_db_hash()?;
    let now = Utc::now().to_rfc3339();

    let metadata = SyncMetadata {
        hash: local_hash,
        last_sync: now.clone(),
    };

    let metadata_url = format!("{}/meta.json", storage_url);
    let response = client
        .put(&metadata_url)
        .header(CONTENT_TYPE, "application/json")
        .json(&metadata)
        .send()
        .await
        .context("Failed to upload metadata")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to upload metadata: {}",
            response.status()
        ));
    }

    let mut config = load_sync_config()?;
    config.last_sync = Some(now);
    save_sync_config(&config)?;

    println!("Database pushed successfully!");
    Ok(())
}

async fn get_remote_metadata(storage_url: &str, token: &str) -> Result<SyncMetadata> {
    let metadata_url = format!("{}/meta.json", storage_url);
    let client = create_client(token)?;

    let response = client
        .get(&metadata_url)
        .send()
        .await
        .context("Failed to fetch metadata")?;

    if response.status().is_success() {
        let metadata: SyncMetadata = response.json().await.context("Failed to parse metadata")?;
        Ok(metadata)
    } else if response.status().as_u16() == 404 || response.status().as_u16() == 400 {
        println!("No metadata found in the remote storage. It has not been initialized yet.");
        println!("Please run the 'gnedby sync push' command first to initialize.");

        Ok(SyncMetadata {
            hash: "empty".to_string(),
            last_sync: "never".to_string(),
        })
    } else {
        Err(anyhow::anyhow!(
            "Failed to get metadata: {}",
            response.status()
        ))
    }
}

fn backup_database() -> Result<()> {
    let db_path = get_db_path()?;

    if !db_path.exists() {
        return Ok(());
    }

    let backup_path = db_path.with_extension("db.bak");
    fs::copy(&db_path, &backup_path).context("Failed to create database backup")?;

    println!("Created backup at {:?}", backup_path);
    Ok(())
}

fn create_client(token: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    Ok(client)
}

pub async fn auto_sync() -> Result<()> {
    let config = load_sync_config()?;

    if !config.auto_sync {
        println!("Auto sync is disabled in configuration");
        return Ok(());
    }

    if config.storage_url.is_none() || config.token.is_none() {
        println!("Auto sync is enabled but storage_url or token is missing");
        return Ok(());
    }

    match push_to_remote().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Auto sync failed during push_to_remote: {}", e);
            Err(e)
        }
    }
}
