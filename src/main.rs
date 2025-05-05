mod cli;
mod db;
mod metadata;

use anyhow::Result;
use cli::{parse_args, Command};
use db::{Album, Database};
use metadata::fetch_album_metadata;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run() -> Result<()> {
    let cli = parse_args()?;
    let db = Database::new()?;

    match cli.command {
        Command::Add { album_id, format } => {
            let metadata = fetch_album_metadata(&album_id).await?;

            let album = Album {
                id: None,
                artist: metadata.artist,
                album: metadata.album,
                genre: metadata.genre,
                release_date: metadata.release_date,
                format,
                source_url: metadata.source_url,
                country: metadata.country,
                artwork_url: metadata.artwork_url,
            };

            let id = db.add_album(&album)?;

            println!(
                "Added album \"{}\" by \"{}\" with ID: {}",
                album.album, album.artist, id
            );
        }
        Command::Show {
            year,
            artist,
            genre,
            format,
            country,
        } => {
            println!(
                "Showing albums with filters: year={:?}, artist={:?}, genre={:?}, format={:?}, country={:?}",
                year, artist, genre, format, country
            );
            // TODO: 데이터베이스에서 앨범 목록 가져오기
        }
        Command::Report {
            year,
            artist,
            genre,
            format,
            country,
        } => {
            println!(
                "Generating report with filters: year={:?}, artist={:?}, genre={:?}, format={:?}, country={:?}",
                year, artist, genre, format, country
            );
            // TODO: 데이터베이스에서 보고서 생성
        }
        Command::Sync { command } => {
            println!("Sync command: {:?}", command);
            // TODO: 동기화 기능 구현
        }
    }

    Ok(())
}
