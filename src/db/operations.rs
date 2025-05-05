use crate::db::models::Album;
use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
            .ok_or_else(|| anyhow::anyhow!("Failed to determine data directory"))?;

        let data_dir = project_dirs.data_dir();
        fs::create_dir_all(data_dir)?;

        let db_path = data_dir.join("albums.db");
        let conn = Connection::open(&db_path)?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS albums (
                id INTEGER PRIMARY KEY,
                artist TEXT NOT NULL,
                album TEXT NOT NULL,
                genre TEXT,
                release_date TEXT,
                format TEXT DEFAULT 'CD',
                source_url TEXT,
                country TEXT,
                artwork_url TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_album(&self, album: &Album) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO albums (artist, album, genre, release_date, format, source_url, country, artwork_url)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &album.artist,
                &album.album,
                &album.genre,
                &album.release_date,
                &album.format,
                &album.source_url,
                &album.country,
                &album.artwork_url,
            ),
        )?;
        Ok(self.conn.last_insert_rowid())
    }
}
