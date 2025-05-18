use crate::db::{Album, Database};
use anyhow::Result;
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

struct AlbumView<'a> {
    artwork_url: &'a str,
    album: &'a str,
    artist: &'a str,
    format_upper: String,
    release_date_short: String,
}

#[derive(Template)]
#[template(path = "albums.html")]
struct AlbumsTemplate<'a> {
    albums: Vec<AlbumView<'a>>,
}

async fn get_albums(State(db): State<Arc<Database>>) -> Json<Vec<Album>> {
    match db.get_all_albums().await {
        Ok(albums) => Json(albums),
        Err(e) => {
            eprintln!("Error fetching albums: {}", e);
            Json(Vec::new())
        }
    }
}

async fn get_album_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> Json<Option<Album>> {
    match db.get_album_by_id(id).await {
        Ok(album) => Json(album),
        Err(e) => {
            eprintln!("Error fetching album: {}", e);
            Json(None)
        }
    }
}

async fn index(State(db): State<Arc<Database>>) -> Html<String> {
    let albums = db.get_all_albums().await.unwrap_or_default();
    let album_views: Vec<AlbumView> = albums
        .iter()
        .map(|album| AlbumView {
            artwork_url: album.artwork_url.as_str(),
            album: album.album.as_str(),
            artist: album.artist.as_str(),
            format_upper: album.format.to_string().to_uppercase(),
            release_date_short: album.release_date.chars().take(10).collect(),
        })
        .collect();
    let tmpl = AlbumsTemplate {
        albums: album_views,
    };
    Html(tmpl.render().unwrap())
}

pub async fn serve() -> Result<()> {
    let db = Database::new().await?;
    let db = Arc::new(db);

    let app = Router::new()
        .route("/", get(index))
        .route("/api/albums", get(get_albums))
        .route("/api/albums/{id}", get(get_album_by_id))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(db);

    let url = "http://localhost:8080";
    println!("Starting server at {}", url);

    if let Err(e) = webbrowser::open(url) {
        eprintln!("Failed to open browser: {}", e);
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
