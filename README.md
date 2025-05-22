# GNEDBY

A CLI tool for managing your CD/LP/USB/Tape collection. Easily add album information using Apple Music links, view lists and summary reports with various filters, and support synchronization using Supabase.

## Features

- Add music albums using Apple Music album IDs
- Add albums manually with interactive prompts
- Automatic album metadata fetching (artist, album title, genre, release date, country, artwork)
- CD/LP/USB/Tape format support
- List view and summary reports with various filters
- Local persistent storage using SQLite
- Token storage with encryption
- Synchronization with Supabase (check/push/pull)
- Automatic synchronization when adding new albums (optional)
- Removal of albums from your collection

## Installation

```bash
cargo install gnedby
```

## Usage

### Adding Albums

```bash
# Add albums using Apple Music IDs
gnedby add <album_id>... [--format <cd|lp|usb|tape>]

# Add album manually with interactive prompts
gnedby manual-add [--format <cd|lp|usb|tape>]
```

### Deleting Albums

```bash
gnedby delete <id>
```

### Viewing Albums

```bash
gnedby show [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <cd|lp|usb|tape>] [--country <country>] [--order-by id|album|artist|year]
```

### Generating Reports

```bash
gnedby report [--year] [--artist] [--genre] [--format] [--country]
```

### Synchronization

```bash
gnedby sync check [-v|--verbose]
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

### Running the Web Server

```bash
gnedby serve
```

- When you run the server, your browser will automatically open at `http://localhost:8080` where you can use the web interface.

### Embedding Artworks

```bash
gnedby embed run [--force]
```

- Downloads artwork image for each album
- Generates a 1280-dimension embedding vector using the MobileNetV2 embedding model
- Uploads to Supabase Vector DB for future image similarity search
- If `--force` is given, all albums are processed; otherwise, only albums without embeddings are processed.

#### Embedding Model Management

```bash
gnedby embed load-model
```

- Downloads the embedding model (ONNX format) from Supabase public storage to your local config directory (`~/.config/gnedby/model.onnx`).
- If the model file already exists, download is skipped.
- This command must be run before embedding if the model is not present.

#### Embedding Configuration

```bash
gnedby embed config show
gnedby embed config set api_url <supabase_vector_api_url>
gnedby embed config set token <supabase_token>
gnedby embed config reset
```

- Manage embedding-related configuration such as Supabase Vector DB API URL and token.
- Configuration is stored in `~/.config/gnedby/embed_config.json`.

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

4. Enable automatic synchronization (optional):

```bash
gnedby sync config set auto_sync true
```

With automatic synchronization enabled, any newly added or deleted albums will be automatically pushed to your Supabase storage. This ensures your collection is always backed up without manual intervention.

## Security Features

- Tokens are stored with XOR encryption using a machine-specific key
- Database backups are created automatically before sync operations
- Integrity checking with SHA-256 hashes

## Tech Stack

- Rust
- clap (CLI parsing)
- rusqlite (database)
- reqwest (HTTP requests)
- base64 and SHA-256 (token encryption)
- Supabase Storage (synchronization)
- axum (web server)
- askama (SSR templates)

## License

MIT License
