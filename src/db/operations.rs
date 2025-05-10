use crate::db::models::Album;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_db_path()?;
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
                format TEXT,
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
                &album.format.as_str(),
                &album.source_url,
                &album.country,
                &album.artwork_url,
            ),
        )?;
        let id = self.conn.last_insert_rowid();

        Ok(id)
    }

    pub fn list_albums(
        &self,
        year: Option<i32>,
        artist: Option<&str>,
        genre: Option<&str>,
        format: Option<&str>,
        country: Option<&str>,
        order_by: Option<&str>,
    ) -> Result<Vec<Album>> {
        let base_query = "
            SELECT id, artist, album, genre, release_date, format, source_url, country, artwork_url 
            FROM albums 
            WHERE 1=1";

        let mut sql = String::from(base_query);
        let mut params_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        let mut add_filter = |condition: &str, value: Box<dyn rusqlite::ToSql>| {
            sql.push_str(condition);
            params_values.push(value);
        };

        if let Some(year_val) = year {
            add_filter(
                " AND strftime('%Y', release_date) = ?",
                Box::new(year_val.to_string()),
            );
        }

        if let Some(artist_val) = artist {
            add_filter(" AND artist LIKE ?", Box::new(format!("%{}%", artist_val)));
        }

        if let Some(genre_val) = genre {
            add_filter(" AND genre LIKE ?", Box::new(format!("%{}%", genre_val)));
        }

        if let Some(format_val) = format {
            add_filter(" AND format = ?", Box::new(format_val));
        }

        if let Some(country_val) = country {
            add_filter(" AND country = ?", Box::new(country_val));
        }

        match order_by.unwrap_or("id") {
            "album" => sql.push_str(" ORDER BY album, artist"),
            "artist" => sql.push_str(" ORDER BY artist, album"),
            "year" => sql.push_str(" ORDER BY release_date DESC, artist, album"),
            _ => sql.push_str(" ORDER BY id"),
        }

        let mut stmt = self.conn.prepare(&sql)?;
        let params_iter = params_values.iter().map(|p| p.as_ref());

        let album_rows = stmt.query_map(rusqlite::params_from_iter(params_iter), |row| {
            Ok(Album {
                id: Some(row.get(0)?),
                artist: row.get(1)?,
                album: row.get(2)?,
                genre: row.get(3)?,
                release_date: row.get(4)?,
                format: row.get(5)?,
                source_url: row.get(6)?,
                country: row.get(7)?,
                artwork_url: row.get(8)?,
            })
        })?;

        let mut albums = Vec::new();
        for album_result in album_rows {
            albums.push(album_result?);
        }

        Ok(albums)
    }

    pub fn get_artist_stats(&self) -> Result<Vec<(String, i64)>> {
        let sql = "
            SELECT artist, COUNT(*) as count 
            FROM albums 
            GROUP BY artist 
            ORDER BY count DESC
        ";

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let artist: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((artist, count))
        })?;

        let mut stats = Vec::new();
        for row_result in rows {
            stats.push(row_result?);
        }

        Ok(stats)
    }

    pub fn get_year_stats(&self) -> Result<Vec<(String, i64)>> {
        let sql = "
            SELECT strftime('%Y', release_date) as year, COUNT(*) as count 
            FROM albums 
            GROUP BY year 
            ORDER BY year ASC
        ";

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let year: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((year, count))
        })?;

        let mut stats = Vec::new();
        for row_result in rows {
            stats.push(row_result?);
        }

        Ok(stats)
    }

    pub fn get_genre_stats(&self) -> Result<Vec<(String, i64)>> {
        let sql = "
            SELECT genre, COUNT(*) as count 
            FROM albums 
            GROUP BY genre 
            ORDER BY count DESC
        ";

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let genre: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((genre, count))
        })?;

        let mut stats = Vec::new();
        for row_result in rows {
            stats.push(row_result?);
        }

        Ok(stats)
    }

    pub fn get_format_stats(&self) -> Result<Vec<(String, i64)>> {
        let sql = "
            SELECT format, COUNT(*) as count 
            FROM albums 
            GROUP BY format 
            ORDER BY count DESC
        ";

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let format: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((format, count))
        })?;

        let mut stats = Vec::new();
        for row_result in rows {
            stats.push(row_result?);
        }

        Ok(stats)
    }

    pub fn get_country_stats(&self) -> Result<Vec<(String, i64)>> {
        let sql = "
            SELECT country, COUNT(*) as count 
            FROM albums 
            GROUP BY country 
            ORDER BY count DESC
        ";

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let country: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((country, count))
        })?;

        let mut stats = Vec::new();
        for row_result in rows {
            stats.push(row_result?);
        }

        Ok(stats)
    }

    pub fn delete_album(&self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute("DELETE FROM albums WHERE id = ?", [id])?;

        if rows_affected == 0 {
            return Err(anyhow::anyhow!("Album with ID {} not found", id));
        }

        Ok(())
    }
}

pub fn get_db_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine data directory"))?;

    let data_dir = project_dirs.data_dir();
    fs::create_dir_all(data_dir)?;

    let db_path = data_dir.join("albums.db");
    Ok(db_path)
}

pub fn calculate_db_hash() -> Result<String> {
    let db_path = get_db_path()?;

    if !db_path.exists() {
        return Ok("empty".to_string());
    }

    let mut file =
        fs::File::open(&db_path).context(format!("Failed to open database file: {:?}", db_path))?;

    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .context("Failed to read database file")?;

    hasher.update(&buffer);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}
