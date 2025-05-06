use clap::{ArgGroup, Parser};

/// A CLI tool for managing your CD/LP collection
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Add a new album to your collection using Apple Music album ID
    Add {
        /// Apple Music album ID (e.g., 1811804666)
        album_id: String,

        /// Album format (CD or LP)
        #[arg(long, default_value = "cd")]
        format: String,
    },
    /// List albums in your collection with optional filters (use only one filter at a time)
    #[command(group(
        ArgGroup::new("filter")
            .args(["year", "artist", "genre", "format", "country"])
            .multiple(false)
            .required(false)
    ))]
    Show {
        /// Filter albums by release year
        #[arg(long)]
        year: Option<i32>,

        /// Filter albums by artist name
        #[arg(long)]
        artist: Option<String>,

        /// Filter albums by genre
        #[arg(long)]
        genre: Option<String>,

        /// Filter albums by format (CD or LP)
        #[arg(long)]
        format: Option<String>,

        /// Filter albums by country
        #[arg(long)]
        country: Option<String>,

        /// Order results by field (id, album, artist, year)
        #[arg(long, default_value = "id")]
        order_by: Option<String>,
    },
    /// Generate a summary report of your collection
    #[command(group(
        ArgGroup::new("filter")
            .args(["year", "artist", "genre", "format", "country"])
            .multiple(false)
            .required(false)
    ))]
    Report {
        /// Filter report by release year
        #[arg(long, default_value_t = true)]
        year: bool,

        /// Filter report by artist name
        #[arg(long)]
        artist: bool,

        /// Filter report by genre
        #[arg(long)]
        genre: bool,

        /// Filter report by format (CD or LP)
        #[arg(long)]
        format: bool,

        /// Filter report by country
        #[arg(long)]
        country: bool,
    },
    /// Synchronize your collection with remote storage
    Sync {
        #[command(subcommand)]
        command: SyncCommand,
    },
}

#[derive(Parser, Debug)]
pub enum SyncCommand {
    /// Check if your local collection is in sync with remote
    Check,
    /// Download changes from remote storage
    Pull,
    /// Upload your local changes to remote storage
    Push,
    /// Configure sync settings
    Config {
        #[command(subcommand)]
        command: SyncConfigCommand,
    },
}

#[derive(Parser, Debug)]
pub enum SyncConfigCommand {
    /// Show current sync configuration
    Show,
    /// Set sync configuration value
    Set {
        /// Configuration key (e.g., storage_url, token)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Reset sync configuration to default values
    Reset,
}
