mod commands;
mod db;
mod stats;
mod ui;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "kitmap")]
#[command(author = "Twilight")]
#[command(version = "0.1.0")]
#[command(about = "A cross-platform CLI for tracking keyboard usage and generating heatmaps")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start listening to keyboard events and recording them
    Listen,
    /// Preview keyboard usage statistics and heatmap
    Preview {
        /// Open web-based visualization instead of ASCII heatmap
        #[arg(short, long)]
        web: bool,
        /// Port for the web server (default: 3456)
        #[arg(short, long, default_value = "3456")]
        port: u16,
    },
    /// Reset all recorded keyboard data
    Reset {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Show the database path
    Db,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Listen => commands::listen::run().await,
        Commands::Preview { web, port } => commands::preview::run(web, port).await,
        Commands::Reset { force } => commands::reset::run(force).await,
        Commands::Db => commands::db::run().await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
