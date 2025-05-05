use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumMetadata {
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub release_date: String,
    pub source_url: String,
    pub artwork_url: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct AppleMusicResponse {
    pub results: Vec<AppleMusicResult>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppleMusicResult {
    #[serde(rename = "artistName")]
    pub artist_name: String,

    #[serde(rename = "collectionName")]
    pub collection_name: String,

    #[serde(rename = "collectionViewUrl")]
    pub collection_view_url: String,

    #[serde(rename = "artworkUrl100")]
    pub artwork_url_100: String,

    pub country: String,

    #[serde(rename = "releaseDate")]
    pub release_date: String,

    #[serde(rename = "primaryGenreName")]
    pub primary_genre_name: String,
}

impl From<AppleMusicResult> for AlbumMetadata {
    fn from(result: AppleMusicResult) -> Self {
        AlbumMetadata {
            artist: result.artist_name,
            album: result.collection_name,
            genre: result.primary_genre_name,
            release_date: result.release_date,
            source_url: result.collection_view_url,
            artwork_url: result.artwork_url_100,
            country: result.country,
        }
    }
}
