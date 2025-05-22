use crate::db::Database;
use crate::embed::models::AlbumVector;
use anyhow::Result;

pub mod downloader;
pub mod model;
pub mod models;
pub mod processor;

use model::EmbeddingModel;
use processor::ImageProcessor;

use crate::api::fetch_embedded_album_ids;
use crate::config::load_embed_config;

pub struct Embedder {
    image_processor: ImageProcessor,
    embedding_model: EmbeddingModel,
}

impl Embedder {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            image_processor: ImageProcessor::new(),
            embedding_model: EmbeddingModel::new().await?,
        })
    }

    pub async fn process_albums(&self, db: &Database, force: bool) -> Result<Vec<AlbumVector>> {
        let albums = db.get_all_albums().await?;
        println!("Found {} albums in local database", albums.len());
        println!("Processing first 5 albums for testing...");

        let mut vectors = Vec::new();
        let albums_to_process = if force {
            albums.iter().take(5).collect::<Vec<_>>()
        } else {
            let config = load_embed_config()?;
            let api_url = config
                .api_url
                .ok_or_else(|| anyhow::anyhow!("api_url is not set."))?;
            let token = config
                .token
                .ok_or_else(|| anyhow::anyhow!("token is not set."))?;
            let embedded_ids = fetch_embedded_album_ids(&api_url, &token).await?;
            albums
                .iter()
                .filter(|a| a.id.is_some() && !embedded_ids.contains(&a.id.unwrap()))
                .take(5)
                .collect::<Vec<_>>()
        };

        for album in albums_to_process {
            println!(
                "\nProcessing artwork for {} by {}",
                album.album, album.artist
            );
            let image_data = self.download_artwork(&album.artwork_url).await?;
            let processed_image = self.image_processor.process_image(&image_data)?;
            let embedding = self.embedding_model.generate_embedding(&processed_image)?;
            let album_vector = AlbumVector::from_album(album, embedding);
            vectors.push(album_vector);
        }
        Ok(vectors)
    }

    async fn download_artwork(&self, url: &str) -> Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
