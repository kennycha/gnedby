mod api;
mod cli;
mod config;
mod db;
mod embed;
mod metadata;
mod sync;
mod web;

use anyhow::Result;
use api::{bulk_upload_album_vectors, fetch_embedded_album_ids, update_album_vector};
use cli::{parse_args, Command, EmbedCommand, EmbedConfigCommand, SyncCommand, SyncConfigCommand};
use comfy_table::{presets::UTF8_BORDERS_ONLY, Cell, CellAlignment, ContentArrangement, Table};
use config::{load_embed_config, load_sync_config, save_embed_config, save_sync_config};
use db::{Album, Database};
use dialoguer::Input;
use embed::{models::AlbumVector, Embedder};
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
    let db = Database::new().await?;

    match cli.command {
        Command::Add { album_ids, format } => {
            for album_id in album_ids {
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

                db.add_album(&album).await?;
                println!("Added album \"{}\" by \"{}\"", album.album, album.artist);
            }

            let config = load_sync_config()?;
            if config.auto_sync && config.storage_url.is_some() && config.token.is_some() {
                match sync::auto_sync().await {
                    Ok(_) => println!("Auto sync completed successfully"),
                    Err(e) => eprintln!("Auto sync failed: {}", e),
                }
            }
        }
        Command::ManualAdd { format } => {
            let artist: String = Input::<String>::new()
                .with_prompt("Enter artist name")
                .allow_empty(false)
                .interact_text()?;

            let album: String = Input::<String>::new()
                .with_prompt("Enter album title")
                .allow_empty(false)
                .interact_text()?;

            let genre: String = Input::<String>::new()
                .with_prompt("Enter genre")
                .allow_empty(false)
                .interact_text()?;

            let release_date: String = Input::<String>::new()
                .with_prompt("Enter release date (YYYY-MM-DD)")
                .allow_empty(false)
                .interact_text()?;

            let country: String = Input::<String>::new()
                .with_prompt("Enter country")
                .allow_empty(false)
                .interact_text()?;

            let source_url: String = Input::<String>::new()
                .with_prompt("Enter source URL")
                .allow_empty(false)
                .interact_text()?;

            let artwork_url: String = Input::<String>::new()
                .with_prompt("Enter artwork URL")
                .allow_empty(false)
                .interact_text()?;

            let album = Album {
                id: None,
                artist,
                album,
                genre,
                release_date,
                format,
                source_url,
                country,
                artwork_url,
            };

            db.add_album(&album).await?;
            println!("Added album \"{}\" by \"{}\"", album.album, album.artist);

            let config = load_sync_config()?;
            if config.auto_sync && config.storage_url.is_some() && config.token.is_some() {
                match sync::auto_sync().await {
                    Ok(_) => println!("Auto sync completed successfully"),
                    Err(e) => eprintln!("Auto sync failed: {}", e),
                }
            }
        }
        Command::Delete { id } => match db.delete_album(id).await {
            Ok(_) => {
                println!("Album with ID {} deleted successfully", id);

                let config = load_sync_config()?;
                if config.auto_sync && config.storage_url.is_some() && config.token.is_some() {
                    match sync::auto_sync().await {
                        Ok(_) => println!("Auto sync completed successfully"),
                        Err(e) => eprintln!("Auto sync failed: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to delete album: {}", e);
            }
        },
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
            let format_ref = format.as_ref().map(|f| f.as_str());
            let country_ref = country.as_deref();
            let order_by_ref = order_by.as_deref();

            let albums = db
                .list_albums(
                    year,
                    artist_ref,
                    genre_ref,
                    format_ref,
                    country_ref,
                    order_by_ref,
                )
                .await?;

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
                    Cell::new(album.format.as_str()),
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
                let artist_stats = db.get_artist_stats().await?;
                create_artist_table(artist_stats)?;
            } else if genre {
                let genre_stats = db.get_genre_stats().await?;
                create_bar_chart_table(genre_stats, "Albums by Genre", "Genre")?;
            } else if format {
                let format_stats = db.get_format_stats().await?;
                create_bar_chart_table(format_stats, "Albums by Format", "Format")?;
            } else if country {
                let country_stats = db.get_country_stats().await?;
                create_bar_chart_table(country_stats, "Albums by Country", "Country")?;
            } else {
                let year_stats = db.get_year_stats().await?;
                create_bar_chart_table(year_stats, "Albums by Year", "Year")?;
            }
        }
        Command::Sync { command } => match command {
            SyncCommand::Check { verbose } => {
                if sync::check_sync_status(verbose).await? {
                    println!("Sync check completed.");
                }
            }
            SyncCommand::Pull => {
                sync::pull_from_remote().await?;
                println!("Sync pull completed.");
            }
            SyncCommand::Push => {
                sync::push_to_remote().await?;
                println!("Sync push completed.");
            }
            SyncCommand::Config { command } => match command {
                SyncConfigCommand::Show => {
                    let config = load_sync_config()?;
                    println!("Current sync configuration:");
                    println!("{}", serde_json::to_string_pretty(&config)?);
                }
                SyncConfigCommand::Set { key, value } => {
                    let mut config = load_sync_config()?;

                    match key.as_str() {
                        "storage_url" => {
                            config.storage_url = Some(value.clone());
                            println!("Set storage_url to: {}", value);
                        }
                        "token" => {
                            config.token = Some(value.clone());
                            println!("Set token to: {}", value);
                        }
                        "auto_sync" => {
                            let auto_sync = value.to_lowercase() == "true";
                            config.auto_sync = auto_sync;
                            println!("Set auto_sync to: {}", auto_sync);

                            if auto_sync {
                                println!("Auto sync is enabled. Albums will be synced to the remote storage automatically.");

                                if config.storage_url.is_none() || config.token.is_none() {
                                    println!("Warning: Sync configuration is not complete. Please set storage_url and token.");
                                }
                            } else {
                                println!("Auto sync is disabled. Manual sync is required with 'gnedby sync push' command.");
                            }
                        }
                        _ => {
                            println!("Unknown sync configuration key: {}", key);
                        }
                    }

                    save_sync_config(&config)?;
                }
                SyncConfigCommand::Reset => {
                    let default_config = config::SyncConfig::default();
                    save_sync_config(&default_config)?;

                    println!("Sync configuration has been reset to default values.");
                    println!("Token has been removed from secure storage.");
                }
            },
        },
        Command::Serve => {
            web::serve().await?;
        }
        Command::Embed { command } => match command {
            EmbedCommand::Run { force } => {
                println!("Starting embedding generation...");
                let embedder = Embedder::new().await?;
                let config = load_embed_config()?;
                let api_url = config
                    .api_url
                    .ok_or_else(|| anyhow::anyhow!("api_url is not set."))?;
                let token = config
                    .token
                    .ok_or_else(|| anyhow::anyhow!("token is not set."))?;

                let albums = db.get_all_albums().await?;
                println!("Found {} albums in local database", albums.len());

                let existing_ids = fetch_embedded_album_ids(&api_url, &token).await?;
                let albums_to_process: Vec<_> = if force {
                    println!("Force mode: processing all albums...");
                    albums.iter().collect()
                } else {
                    println!("Processing only albums without embeddings...");
                    albums
                        .iter()
                        .filter(|a| a.id.is_some() && !existing_ids.contains(&a.id.unwrap()))
                        .collect()
                };

                let vectors = embedder.process_albums(&albums_to_process).await?;
                println!("Embedding generation completed. Uploading to Supabase...");

                if force {
                    let (to_create, to_update): (Vec<_>, Vec<_>) =
                        vectors.iter().partition(|v| !existing_ids.contains(&v.id));

                    if !to_create.is_empty() {
                        let to_create_vec: Vec<AlbumVector> =
                            to_create.into_iter().cloned().collect();
                        bulk_upload_album_vectors(&api_url, &token, &to_create_vec).await?;
                    }

                    if !to_update.is_empty() {
                        println!("Updating {} existing embeddings...", to_update.len());
                        for vector in to_update {
                            update_album_vector(&api_url, &token, vector).await?;
                        }
                    }
                } else {
                    println!("Uploading {} new embeddings...", vectors.len());
                    bulk_upload_album_vectors(&api_url, &token, &vectors).await?;
                }

                println!("Embedding generation & upload completed successfully");
            }
            EmbedCommand::LoadModel => {
                let path = embed::downloader::download_model().await?;
                println!("Model downloaded successfully to: {}", path.display());
            }
            EmbedCommand::Config { command } => match command {
                EmbedConfigCommand::Show => {
                    let config = load_embed_config()?;
                    println!("Current embedding configuration:");
                    println!("{}", serde_json::to_string_pretty(&config)?);
                }
                EmbedConfigCommand::Set { key, value } => {
                    let mut config = load_embed_config()?;
                    match key.as_str() {
                        "api_url" => {
                            config.api_url = Some(value.clone());
                            println!("Set api_url to: {}", value);
                        }
                        "token" => {
                            config.token = Some(value.clone());
                            println!("Set token to: {}", value);
                        }
                        _ => {
                            println!("Unknown embedding configuration key: {}", key);
                        }
                    }
                    save_embed_config(&config)?;
                }
                EmbedConfigCommand::Reset => {
                    let default_config = config::EmbedConfig::default();
                    save_embed_config(&default_config)?;

                    println!("Embedding configuration has been reset to default values.");
                }
            },
        },
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
