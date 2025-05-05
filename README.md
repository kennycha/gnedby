# GNEDBY

A CLI tool for managing your CD/LP collection. Easily add album information using Apple Music links, view lists and summary reports with various filters, and support synchronization using Supabase.

## Features

- Add music albums using Apple Music album IDs
- Automatic album metadata fetching (artist, album title, genre, release date)
- CD/LP format support (`--format lp` flag)
- List view and summary reports with various filters
- Local persistent storage using SQLite
- Synchronization with Supabase (check/push/pull)

## Installation

```bash
cargo install gnedby
```

## Usage

### Adding Albums

```bash
gnedby add <album_id> [--format lp]
```

### Viewing Albums

```bash
gnedby show [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <CD|LP>]
```

### Generating Reports

```bash
gnedby report [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <CD|LP>]
```

### Synchronization

```bash
gnedby sync check
gnedby sync pull
gnedby sync push
```

### Configuration

```bash
gnedby config set storage_url <supabase_url>
gnedby config set token <supabase_token>
```

## Sync Setup

Use Supabase Storage for safe synchronization across multiple devices.

1. Login with Supabase token:

```bash
gnedby login <supabase_token>
```

2. Set storage URL:

```bash
gnedby config set storage_url https://<project>.supabase.co/storage/v1/object/gnedby-sync
```

## Tech Stack

- Rust
- clap (CLI parsing)
- rusqlite (database)
- reqwest (HTTP requests)
- Supabase Storage (synchronization)

## License

MIT License
