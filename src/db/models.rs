use crate::db::DbConnection;
use anyhow::Result;
use chrono::{DateTime, Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub id: Option<i64>,
    pub key_code: String,
    pub key_name: String,
    pub is_modifier: bool,
    pub timestamp: DateTime<Local>,
}

impl KeyEvent {
    pub fn new(key_code: String, key_name: String, is_modifier: bool) -> Self {
        Self {
            id: None,
            key_code,
            key_name,
            is_modifier,
            timestamp: Local::now(),
        }
    }

    pub fn save(&self, db: &DbConnection) -> Result<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO key_events (key_code, key_name, is_modifier, timestamp, hour, day_of_week)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &self.key_code,
                &self.key_name,
                self.is_modifier as i32,
                self.timestamp.to_rfc3339(),
                self.timestamp.hour() as i32,
                self.timestamp.weekday().num_days_from_monday() as i32,
            ),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCombo {
    pub id: Option<i64>,
    pub combo: String,
    pub timestamp: DateTime<Local>,
}

impl KeyCombo {
    pub fn new(combo: String) -> Self {
        Self {
            id: None,
            combo,
            timestamp: Local::now(),
        }
    }

    pub fn save(&self, db: &DbConnection) -> Result<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO key_combos (combo, timestamp) VALUES (?1, ?2)",
            (&self.combo, self.timestamp.to_rfc3339()),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<i64>,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub total_keys: i64,
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: None,
            start_time: Local::now(),
            end_time: None,
            total_keys: 0,
        }
    }

    pub fn start(&mut self, db: &DbConnection) -> Result<i64> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO sessions (start_time, total_keys) VALUES (?1, ?2)",
            (self.start_time.to_rfc3339(), self.total_keys),
        )?;
        let id = conn.last_insert_rowid();
        self.id = Some(id);
        Ok(id)
    }

    pub fn end(&mut self, db: &DbConnection) -> Result<()> {
        self.end_time = Some(Local::now());
        if let Some(id) = self.id {
            let conn = db.lock().unwrap();
            conn.execute(
                "UPDATE sessions SET end_time = ?1, total_keys = ?2 WHERE id = ?3",
                (self.end_time.unwrap().to_rfc3339(), self.total_keys, id),
            )?;
        }
        Ok(())
    }

    pub fn increment_keys(&mut self) {
        self.total_keys += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingSample {
    pub chars_per_minute: f64,
    pub timestamp: DateTime<Local>,
}

impl TypingSample {
    pub fn new(chars_per_minute: f64) -> Self {
        Self {
            chars_per_minute,
            timestamp: Local::now(),
        }
    }

    pub fn save(&self, db: &DbConnection) -> Result<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO typing_samples (chars_per_minute, timestamp) VALUES (?1, ?2)",
            (self.chars_per_minute, self.timestamp.to_rfc3339()),
        )?;
        Ok(())
    }
}
