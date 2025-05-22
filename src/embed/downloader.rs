use anyhow::{Context, Result};
use directories::BaseDirs;
use reqwest::Client;
use std::path::PathBuf;
use tokio::fs;

const MODEL_URL: &str =
    "https://xqthuirzlhsygclcfuqf.supabase.co/storage/v1/object/public/gnedby-embed/model.onnx";

pub async fn download_model() -> Result<PathBuf> {
    let config_dir = BaseDirs::new()
        .context("Failed to get config directory")?
        .config_dir()
        .join("gnedby");

    fs::create_dir_all(&config_dir)
        .await
        .context("Failed to create config directory")?;

    let model_path = config_dir.join("model.onnx");

    // Skip download if model already exists
    if model_path.exists() {
        println!("Model already exists at: {}", model_path.display());
        return Ok(model_path);
    }

    println!("Downloading model from: {}", MODEL_URL);

    let client = Client::new();
    let response = client
        .get(MODEL_URL)
        .send()
        .await
        .context("Failed to download model")?;

    let bytes = response
        .bytes()
        .await
        .context("Failed to get model bytes")?;

    fs::write(&model_path, bytes)
        .await
        .context("Failed to save model")?;

    println!("Model downloaded to: {}", model_path.display());

    Ok(model_path)
}
