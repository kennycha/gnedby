use clap::Parser;

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
    /// List albums in your collection with optional filters
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
    },
    /// Generate a summary report of your collection
    Report {
        /// Filter report by release year
        #[arg(long)]
        year: Option<i32>,

        /// Filter report by artist name
        #[arg(long)]
        artist: Option<String>,

        /// Filter report by genre
        #[arg(long)]
        genre: Option<String>,

        /// Filter report by format (CD or LP)
        #[arg(long)]
        format: Option<String>,
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
}
