use anyhow::Result;
use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<()> {
    // Key events table - stores individual key presses
    conn.execute(
        "CREATE TABLE IF NOT EXISTS key_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key_code TEXT NOT NULL,
            key_name TEXT NOT NULL,
            is_modifier INTEGER NOT NULL DEFAULT 0,
            timestamp TEXT NOT NULL,
            hour INTEGER NOT NULL,
            day_of_week INTEGER NOT NULL
        )",
        [],
    )?;

    // Key combinations table - stores modifier + key combinations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS key_combos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            combo TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    // Sessions table - tracks recording sessions
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time TEXT NOT NULL,
            end_time TEXT,
            total_keys INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    // Typing speed samples - for calculating WPM
    conn.execute(
        "CREATE TABLE IF NOT EXISTS typing_samples (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            chars_per_minute REAL NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    // Create indexes for better query performance
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_key_events_key_name ON key_events(key_name);
         CREATE INDEX IF NOT EXISTS idx_key_events_timestamp ON key_events(timestamp);
         CREATE INDEX IF NOT EXISTS idx_key_events_hour ON key_events(hour);
         CREATE INDEX IF NOT EXISTS idx_key_combos_combo ON key_combos(combo);
         CREATE INDEX IF NOT EXISTS idx_typing_samples_timestamp ON typing_samples(timestamp);",
    )?;

    Ok(())
}

pub fn clear_all_data(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "DELETE FROM key_events;
         DELETE FROM key_combos;
         DELETE FROM sessions;
         DELETE FROM typing_samples;
         VACUUM;",
    )?;
    Ok(())
}
