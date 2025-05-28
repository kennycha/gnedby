use crate::embed::models::AlbumVector;
use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

pub async fn bulk_upload_album_vectors(
    api_url: &str,
    token: &str,
    vectors: &[AlbumVector],
) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/album_vectors", api_url.trim_end_matches('/'));

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("apikey", HeaderValue::from_str(token)?);

    let res = client
        .post(&url)
        .headers(headers)
        .json(vectors)
        .send()
        .await
        .context("Failed to send bulk request to Supabase")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("Supabase API error (bulk): {} - {}", status, text);
    }

    Ok(())
}

pub async fn fetch_embedded_album_ids(api_url: &str, token: &str) -> Result<Vec<i64>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/album_vectors?id=not.is.null&select=id",
        api_url.trim_end_matches('/')
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("apikey", HeaderValue::from_str(token)?);

    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .context("Failed to fetch embedded album ids from Supabase")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("Supabase API error (fetch ids): {} - {}", status, text);
    }

    let ids: Vec<serde_json::Value> = res.json().await?;
    let mut result = Vec::new();
    for v in ids {
        if let Some(id) = v.get("id").and_then(|x| x.as_i64()) {
            result.push(id);
        }
    }
    Ok(result)
}

pub async fn update_album_vector(api_url: &str, token: &str, vector: &AlbumVector) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/album_vectors?id=eq.{}",
        api_url.trim_end_matches('/'),
        vector.id
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("apikey", HeaderValue::from_str(token)?);

    let res = client
        .patch(&url)
        .headers(headers)
        .json(vector)
        .send()
        .await
        .context("Failed to send update request to Supabase")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("Supabase API error (update): {} - {}", status, text);
    }

    Ok(())
}
