pub mod models;
pub mod schema;

use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

/// Get the database path in the user's data directory
pub fn get_db_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "twilight", "kitmap") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        Ok(data_dir.join("kitmap.db"))
    } else {
        Ok(PathBuf::from("kitmap.db"))
    }
}

/// Initialize the database connection and create tables
pub fn init_db() -> Result<DbConnection> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    // Enable WAL mode for better concurrent performance
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;

    schema::create_tables(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}

/// Initialize an in-memory database for testing
#[cfg(test)]
pub fn init_test_db() -> Result<DbConnection> {
    let conn = Connection::open_in_memory()?;
    schema::create_tables(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}
