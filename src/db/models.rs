use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<i64>,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub release_date: String,
    pub format: String,
    pub source_url: String,
    pub country: String,
    pub artwork_url: String,
}
