use crate::db::models::Album;
use crate::embed::models::AlbumVector;
use anyhow::Result;

pub mod downloader;
pub mod model;
pub mod models;
pub mod processor;

use model::EmbeddingModel;
use processor::ImageProcessor;

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

    pub async fn process_albums(&self, albums: &[&Album]) -> Result<Vec<AlbumVector>> {
        println!("Processing {} albums...", albums.len());

        let mut vectors = Vec::new();
        for album in albums {
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
