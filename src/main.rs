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

fn create_bar_chart_table<T: AsRef<str> + std::fmt::Display>(
    stats: Vec<(T, i64)>,
    title: &str,
    column_name: &str,
) -> Result<()> {
    if stats.is_empty() {
        println!("No albums found in my GNEDBY");
        return Ok(());
    }

    println!("\n                         {}\n", title);

    let max_count = stats.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let max_bar_length = 50;

    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new(column_name).set_alignment(CellAlignment::Left),
            Cell::new("Count").set_alignment(CellAlignment::Left),
            Cell::new("Bar").set_alignment(CellAlignment::Left),
        ]);

    for (item, count) in stats {
        let bar_length =
            ((count as f64 / max_count as f64) * max_bar_length as f64).round() as usize;
        let bar = "▄".repeat(bar_length);

        table.add_row(vec![
            Cell::new(item),
            Cell::new(count.to_string()),
            Cell::new(bar),
        ]);
    }

    println!("{table}");
    Ok(())
}

fn create_artist_table(stats: Vec<(String, i64)>) -> Result<()> {
    if stats.is_empty() {
        println!("No albums found in my GNEDBY");
        return Ok(());
    }

    println!("\n         Most Popular Artists\n");

    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Artist").set_alignment(CellAlignment::Left),
            Cell::new("Albums Count").set_alignment(CellAlignment::Left),
        ]);

    for (artist, count) in stats.iter().take(13) {
        table.add_row(vec![Cell::new(artist), Cell::new(count.to_string())]);
    }

    println!("{table}");
    Ok(())
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
            year: _,
            artist,
            genre,
            format,
            country,
        } => {
            if artist {
                let artist_stats = db.get_artist_stats()?;
                create_artist_table(artist_stats)?;
            } else if genre {
                let genre_stats = db.get_genre_stats()?;
                create_bar_chart_table(genre_stats, "Albums by Genre", "Genre")?;
            } else if format {
                let format_stats = db.get_format_stats()?;
                create_bar_chart_table(format_stats, "Albums by Format", "Format")?;
            } else if country {
                let country_stats = db.get_country_stats()?;
                create_bar_chart_table(country_stats, "Albums by Country", "Country")?;
            } else {
                let year_stats = db.get_year_stats()?;
                create_bar_chart_table(year_stats, "Albums by Year", "Year")?;
            }
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
