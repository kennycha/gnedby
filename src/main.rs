mod cli;
mod db;
mod metadata;

use anyhow::Result;
use cli::{parse_args, Command};
use comfy_table::{presets::UTF8_BORDERS_ONLY, Cell, CellAlignment, ContentArrangement, Table};
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
                format: format.to_lowercase(),
                source_url: metadata.source_url,
                country: metadata.country,
                artwork_url: metadata.artwork_url,
            };

            db.add_album(&album)?;
            println!("Added album \"{}\" by \"{}\"", album.album, album.artist);
        }
        Command::Show {
            year,
            artist,
            genre,
            format,
            country,
            order_by,
        } => {
            let filter_msg = if let Some(year) = year {
                format!("by year: {}", year)
            } else if let Some(artist) = &artist {
                format!("by artist: {}", artist)
            } else if let Some(genre) = &genre {
                format!("by genre: {}", genre)
            } else if let Some(format) = &format {
                format!("by format: {}", format)
            } else if let Some(country) = &country {
                format!("by country: {}", country)
            } else {
                "all albums".to_string()
            };

            let artist_ref = artist.as_deref();
            let genre_ref = genre.as_deref();
            let format_ref = format.as_deref();
            let country_ref = country.as_deref();
            let order_by_ref = order_by.as_deref();

            let albums = db.list_albums(
                year,
                artist_ref,
                genre_ref,
                format_ref,
                country_ref,
                order_by_ref,
            )?;

            if albums.is_empty() {
                println!("No albums found in my GNEDBY {}", filter_msg);
                return Ok(());
            }

            let mut table = Table::new();
            table
                .load_preset(UTF8_BORDERS_ONLY)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("ID").set_alignment(CellAlignment::Center),
                    Cell::new("Album").set_alignment(CellAlignment::Center),
                    Cell::new("Artist").set_alignment(CellAlignment::Center),
                    Cell::new("Genre").set_alignment(CellAlignment::Center),
                    Cell::new("Country").set_alignment(CellAlignment::Center),
                    Cell::new("Format").set_alignment(CellAlignment::Center),
                    Cell::new("Year").set_alignment(CellAlignment::Center),
                ]);

            for album in &albums {
                let year = extract_year(&album.release_date);

                table.add_row(vec![
                    Cell::new(album.id.unwrap_or(0).to_string()),
                    Cell::new(&album.album),
                    Cell::new(&album.artist),
                    Cell::new(&album.genre),
                    Cell::new(&album.country),
                    Cell::new(&album.format),
                    Cell::new(year),
                ]);
            }

            println!("In my GNEDBY, {}", filter_msg);
            println!("{table}");
            println!("{} album(s) found", albums.len());
        }
        Command::Report {
            year,
            artist,
            genre,
            format,
            country,
        } => {
            let filter_msg = if let Some(year) = year {
                format!("for year: {}", year)
            } else if let Some(artist) = &artist {
                format!("for artist: {}", artist)
            } else if let Some(genre) = &genre {
                format!("for genre: {}", genre)
            } else if let Some(format) = &format {
                format!("for format: {}", format)
            } else if let Some(country) = &country {
                format!("for country: {}", country)
            } else {
                "for all albums".to_string()
            };

            println!("Generating report in my GNEDBY {}", filter_msg);
        }
        Command::Sync { command } => {
            println!("Sync command: {:?}", command);
        }
    }

    Ok(())
}

fn extract_year(date_str: &str) -> &str {
    if let Some(idx) = date_str.find('-') {
        &date_str[0..idx]
    } else if date_str.len() >= 4 {
        &date_str[0..4]
    } else {
        date_str
    }
}
