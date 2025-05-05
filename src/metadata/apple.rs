use crate::metadata::models::{AlbumMetadata, AppleMusicResponse};
use anyhow::{Context, Result};
use reqwest::Client;

const ITUNES_API_URL: &str = "https://itunes.apple.com/lookup";

pub async fn fetch_album_metadata(album_id: &str) -> Result<AlbumMetadata> {
    let client = Client::new();

    let response = client
        .get(ITUNES_API_URL)
        .query(&[("id", album_id), ("entity", "album")])
        .send()
        .await
        .context("Failed to fetch album metadata from Apple Music")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!(
            "Apple Music API returned error status: {} - {}",
            status.as_u16(),
            status.to_string()
        );
    }

    let api_response: AppleMusicResponse = response
        .json()
        .await
        .context("Failed to parse Apple Music API response")?;

    if api_response.results.is_empty() {
        anyhow::bail!("No album found with ID: {}", album_id);
    }

    let first_result = api_response.results[0].clone();
    Ok(AlbumMetadata::from(first_result))
}
