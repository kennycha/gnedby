use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumVector {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub artwork_url: String,
    pub embedding: Vec<f32>,
}

impl AlbumVector {
    pub fn from_album(album: &crate::db::models::Album, embedding: Vec<f32>) -> Self {
        Self {
            id: album.id.unwrap_or(0),
            title: album.album.clone(),
            artist: album.artist.clone(),
            artwork_url: album.artwork_url.clone(),
            embedding,
        }
    }
}
