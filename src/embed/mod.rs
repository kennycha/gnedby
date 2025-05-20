use crate::db::Database;
use anyhow::Result;

pub mod model;
mod processor;

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

    pub async fn process_albums(&self, db: &Database, _force: bool) -> Result<()> {
        let albums = db.get_all_albums().await?;

        println!("Found {} albums in local database", albums.len());
        println!("Processing first 5 albums for testing...");

        // 앞에서 5개 앨범만 처리
        for album in albums.iter().take(5) {
            println!(
                "\nProcessing artwork for {} by {}",
                album.album, album.artist
            );

            // Download artwork
            let image_data = self.download_artwork(&album.artwork_url).await?;

            // Process image
            let processed_image = self.image_processor.process_image(&image_data)?;

            // Generate embedding
            let embedding = self.embedding_model.generate_embedding(&processed_image)?;

            // 임베딩 검증
            let min = embedding.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max = embedding.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let mean = embedding.iter().sum::<f32>() / embedding.len() as f32;
            let norm = (embedding.iter().map(|x| x * x).sum::<f32>()).sqrt();

            println!("Embedding stats:");
            println!("  Size: {}", embedding.len());
            println!("  Min: {:.4}", min);
            println!("  Max: {:.4}", max);
            println!("  Mean: {:.4}", mean);
            println!("  Norm: {:.4}", norm);
            println!("  First 5 values: {:?}", &embedding[..5]);
        }

        Ok(())
    }

    async fn download_artwork(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
