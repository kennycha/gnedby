# GNEDBY

A CLI tool for managing your CD/LP collection. Easily add album information using Apple Music links, view lists and summary reports with various filters, and support synchronization using Supabase.

## Features

- Add music albums using Apple Music album IDs
- Automatic album metadata fetching (artist, album title, genre, release date, country, artwork)
- CD/LP format support (`--format lp` flag)
- List view and summary reports with various filters
- Local persistent storage using SQLite
- Secure token storage using system keychain
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
gnedby show [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <CD|LP>] [--country <country>] [--order-by id|album|artist|year]
```

### Generating Reports

```bash
gnedby report [--year] [--artist] [--genre] [--format] [--country]
```

### Synchronization

```bash
gnedby sync check
gnedby sync pull
gnedby sync push
```

### Configuration

```bash
gnedby sync config show
gnedby sync config set storage_url <supabase_url>
gnedby sync config set token <supabase_token>
gnedby sync config set auto_sync true|false
gnedby sync config reset
```

## Sync Setup

Use Supabase Storage for safe synchronization across multiple devices.

1. Create a bucket in Supabase Storage:

   - Log in to Supabase dashboard
   - Go to Storage and create a new bucket named `gnedby-sync`
   - Configure appropriate bucket policies (for service_role access)

2. Configure sync settings:

```bash
gnedby sync config set storage_url https://your-project-id.supabase.co/storage/v1/object/gnedby-sync

gnedby sync config set token eyJhbGc...your-token

gnedby sync push
```

3. On other devices, configure the same settings and use `gnedby sync pull` to get your data.

## Security Features

- Tokens are securely stored in your system's keychain/credential store
- Database backups are created automatically before sync operations
- Integrity checking with SHA-256 hashes

## Tech Stack

- Rust
- clap (CLI parsing)
- rusqlite (database)
- reqwest (HTTP requests)
- keyring (secure token storage)
- Supabase Storage (synchronization)

## License

MIT License
