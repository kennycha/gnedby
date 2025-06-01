# GNEDBY

> After accidentally buying a duplicate CD, I realized there was a problem to solve

## What is GNEDBY?

- GNEDBY is a CLI tool for digitizing a physical music collection.
- GNEDBY was named after IKEA's "GNEDBY" shelf.

## Installation

```bash
cargo install gnedby --locked
```

⚠️ **Note:** I recommend using the `--locked` flag to ensure dependency compatibility.

## Basic Usage

### Add Albums

Add music albums using Apple Music's album IDs:

```bash
gnedby add <album_id>... [--format <cd|lp|usb|tape>]
```

Add albums manually if they're not available on Apple Music:

```bash
gnedby manual-add [--format <cd|lp|usb|tape>]
```

### View Your Collection

Display albums with various filters:

```bash
gnedby show [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <cd|lp|usb|tape>] [--country <country>] [--order-by id|album|artist|year]
```

Generate collection reports:

```bash
gnedby report [--year] [--artist] [--genre] [--format] [--country]
```

### Manage Albums

Remove an album:

```bash
gnedby delete <id>
```

## Advanced Features

### Synchronization

Check sync status:

```bash
gnedby sync check [-v|--verbose]
```

Sync with remote storage:

```bash
gnedby sync pull
gnedby sync push
```

Configure sync settings:

```bash
gnedby sync config show
gnedby sync config set storage_url <supabase_url>
gnedby sync config set token <supabase_token>
gnedby sync config set auto_sync true|false
gnedby sync config reset
```

### Web Interface

View your collection in a browser:

```bash
gnedby serve
```

## Album Artwork Embedding

For use with the "Is It In My GNEDBY?" web app:

Generate artwork embeddings:

```bash
gnedby embed run [--force]
```

Load the embedding model:

```bash
gnedby embed load-model
```

Configure embedding settings:

```bash
gnedby embed config show
gnedby embed config set api_url <supabase_vector_api_url>
gnedby embed config set token <supabase_token>
gnedby embed config reset
```

## My Personal Use Case

I built a companion web app to check if an album is already in my collection before buying:

- Web app: https://kennycha.github.io/iii-my-gnedby/
- Source code: https://github.com/kennycha/iii-my-gnedby
