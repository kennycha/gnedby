# GNEDBY

> After buying a CD that was already in my GNEDBY, I thought there is a problem to solve

## What is GNEDBY?

- GNEDBY is a CLI tool for digitizing physical music library.
- GNEDBY was named after IKEA's "GNEDBY".

## Installation

```bash
cargo install gnedby
```

## Basic Usage

- Add music albums using Apple Music's album IDs

```bash
gnedby add <album_id>... [--format <cd|lp|usb|tape>]
```

- Add music album manually, if Apple Music doesn't serve it

```bash
gnedby manual-add [--format <cd|lp|usb|tape>]
```

- View albums

```bash
gnedby show [--year <YYYY>] [--artist <name>] [--genre <genre>] [--format <cd|lp|usb|tape>] [--country <country>] [--order-by id|album|artist|year]
```

- View reports

```bash
gnedby report [--year] [--artist] [--genre] [--format] [--country]
```

- Remove album

```bash
gnedby delete <id>
```

## Further Usage

- Synchronization

```bash
gnedby sync check [-v|--verbose]
gnedby sync pull
gnedby sync push
```

- Configuration for sync

```bash
gnedby sync config show
gnedby sync config set storage_url <supabase_url>
gnedby sync config set token <supabase_token>
gnedby sync config set auto_sync true|false
gnedby sync config reset
```

- View albums in browser

```bash
gnedby serve
```

## And I Use This Project Like:

- Embedding artworks

```bash
gnedby embed run [--force]
```

- Load embedding model

```bash
gnedby embed load-model
```

- Configuration for embed

```bash
gnedby embed config show
gnedby embed config set api_url <supabase_vector_api_url>
gnedby embed config set token <supabase_token>
gnedby embed config reset
```

- Check if it's in my GNEDBY

  - https://kennycha.github.io/iii-my-gnedby/
  - https://github.com/kennycha/iii-my-gnedby
