use crate::db::DbConnection;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStats {
    pub key_name: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboStats {
    pub combo: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyStats {
    pub hour: i32,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub day: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllStats {
    pub total_keys: i64,
    pub total_combos: i64,
    pub total_sessions: i64,
    pub total_time_minutes: f64,
    pub most_pressed_key: Option<KeyStats>,
    pub most_pressed_combo: Option<ComboStats>,
    pub top_keys: Vec<KeyStats>,
    pub top_combos: Vec<ComboStats>,
    pub spacebar_count: i64,
    pub enter_count: i64,
    pub backspace_count: i64,
    pub delete_count: i64,
    pub escape_count: i64,
    pub tab_count: i64,
    pub arrow_keys_count: i64,
    pub modifier_keys_count: i64,
    pub letter_keys_count: i64,
    pub number_keys_count: i64,
    pub special_keys_count: i64,
    pub hourly_distribution: Vec<HourlyStats>,
    pub daily_distribution: Vec<DailyStats>,
    pub most_active_hour: Option<HourlyStats>,
    pub most_active_day: Option<DailyStats>,
    pub average_keys_per_session: f64,
    pub average_typing_speed: f64,
    pub max_typing_speed: f64,
    pub key_frequency_map: HashMap<String, i64>,
    pub first_recorded: Option<String>,
    pub last_recorded: Option<String>,
    pub unique_keys_used: i64,
    pub keys_per_minute_avg: f64,
}

pub struct StatsCalculator {
    db: DbConnection,
}

impl StatsCalculator {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub fn calculate_all(&self) -> Result<AllStats> {
        let conn = self.db.lock().unwrap();

        // Total keys
        let total_keys: i64 =
            conn.query_row("SELECT COUNT(*) FROM key_events", [], |row| row.get(0))?;

        // Total combos
        let total_combos: i64 =
            conn.query_row("SELECT COUNT(*) FROM key_combos", [], |row| row.get(0))?;

        // Total sessions
        let total_sessions: i64 =
            conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;

        // Total time from sessions (in minutes)
        let total_time_minutes: f64 = conn.query_row(
            "SELECT COALESCE(
                SUM(
                    CAST((julianday(end_time) - julianday(start_time)) * 24 * 60 AS REAL)
                ), 0.0
            ) FROM sessions WHERE end_time IS NOT NULL",
            [],
            |row| row.get(0),
        )?;

        // Most pressed key
        let most_pressed_key = self.get_most_pressed_key(&conn)?;

        // Most pressed combo
        let most_pressed_combo = self.get_most_pressed_combo(&conn)?;

        // Top 20 keys
        let top_keys = self.get_top_keys(&conn, 20, total_keys)?;

        // Top 10 combos
        let top_combos = self.get_top_combos(&conn, 10)?;

        // Special key counts
        let spacebar_count = self.get_key_count(&conn, "Space")?;
        let enter_count =
            self.get_key_count(&conn, "Return")? + self.get_key_count(&conn, "Enter")?;
        let backspace_count = self.get_key_count(&conn, "Backspace")?;
        let delete_count = self.get_key_count(&conn, "Delete")?;
        let escape_count = self.get_key_count(&conn, "Escape")?;
        let tab_count = self.get_key_count(&conn, "Tab")?;

        // Arrow keys count
        let arrow_keys_count = self.get_key_count(&conn, "UpArrow")?
            + self.get_key_count(&conn, "DownArrow")?
            + self.get_key_count(&conn, "LeftArrow")?
            + self.get_key_count(&conn, "RightArrow")?;

        // Modifier keys count
        let modifier_keys_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM key_events WHERE is_modifier = 1",
            [],
            |row| row.get(0),
        )?;

        // Letter keys count
        let letter_keys_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM key_events WHERE key_name GLOB '[A-Za-z]'",
            [],
            |row| row.get(0),
        )?;

        // Number keys count
        let number_keys_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM key_events WHERE key_name GLOB '[0-9]' OR key_name LIKE 'Num%' OR key_name LIKE 'Key%'",
            [],
            |row| row.get(0),
        )?;

        // Special keys count (everything else)
        let special_keys_count =
            total_keys - letter_keys_count - number_keys_count - modifier_keys_count;

        // Hourly distribution
        let hourly_distribution = self.get_hourly_distribution(&conn)?;

        // Daily distribution
        let daily_distribution = self.get_daily_distribution(&conn)?;

        // Most active hour
        let most_active_hour = hourly_distribution.iter().max_by_key(|h| h.count).cloned();

        // Most active day
        let most_active_day = daily_distribution.iter().max_by_key(|d| d.count).cloned();

        // Average keys per session
        let average_keys_per_session = if total_sessions > 0 {
            total_keys as f64 / total_sessions as f64
        } else {
            0.0
        };

        // Typing speed statistics
        let (average_typing_speed, max_typing_speed) = self.get_typing_speed_stats(&conn)?;

        // Key frequency map for heatmap
        let key_frequency_map = self.get_key_frequency_map(&conn)?;

        // First and last recorded timestamps
        let first_recorded = self.get_first_recorded(&conn)?;
        let last_recorded = self.get_last_recorded(&conn)?;

        // Unique keys used
        let unique_keys_used: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT key_name) FROM key_events",
            [],
            |row| row.get(0),
        )?;

        // Keys per minute average
        let keys_per_minute_avg = if total_time_minutes > 0.0 {
            total_keys as f64 / total_time_minutes
        } else {
            0.0
        };

        Ok(AllStats {
            total_keys,
            total_combos,
            total_sessions,
            total_time_minutes,
            most_pressed_key,
            most_pressed_combo,
            top_keys,
            top_combos,
            spacebar_count,
            enter_count,
            backspace_count,
            delete_count,
            escape_count,
            tab_count,
            arrow_keys_count,
            modifier_keys_count,
            letter_keys_count,
            number_keys_count,
            special_keys_count,
            hourly_distribution,
            daily_distribution,
            most_active_hour,
            most_active_day,
            average_keys_per_session,
            average_typing_speed,
            max_typing_speed,
            key_frequency_map,
            first_recorded,
            last_recorded,
            unique_keys_used,
            keys_per_minute_avg,
        })
    }

    fn get_most_pressed_key(&self, conn: &rusqlite::Connection) -> Result<Option<KeyStats>> {
        let total: i64 = conn.query_row("SELECT COUNT(*) FROM key_events", [], |row| row.get(0))?;

        let result: Option<(String, i64)> = conn
            .query_row(
                "SELECT key_name, COUNT(*) as cnt FROM key_events 
             GROUP BY key_name ORDER BY cnt DESC LIMIT 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .ok();

        Ok(result.map(|(key_name, count)| KeyStats {
            key_name,
            count,
            percentage: if total > 0 {
                (count as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }))
    }

    fn get_most_pressed_combo(&self, conn: &rusqlite::Connection) -> Result<Option<ComboStats>> {
        let result: Option<(String, i64)> = conn
            .query_row(
                "SELECT combo, COUNT(*) as cnt FROM key_combos 
             GROUP BY combo ORDER BY cnt DESC LIMIT 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .ok();

        Ok(result.map(|(combo, count)| ComboStats { combo, count }))
    }

    fn get_top_keys(
        &self,
        conn: &rusqlite::Connection,
        limit: usize,
        total: i64,
    ) -> Result<Vec<KeyStats>> {
        let mut stmt = conn.prepare(
            "SELECT key_name, COUNT(*) as cnt FROM key_events 
             GROUP BY key_name ORDER BY cnt DESC LIMIT ?1",
        )?;

        let keys = stmt.query_map([limit as i64], |row| {
            let key_name: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok(KeyStats {
                key_name,
                count,
                percentage: if total > 0 {
                    (count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
        })?;

        Ok(keys.filter_map(|k| k.ok()).collect())
    }

    fn get_top_combos(&self, conn: &rusqlite::Connection, limit: usize) -> Result<Vec<ComboStats>> {
        let mut stmt = conn.prepare(
            "SELECT combo, COUNT(*) as cnt FROM key_combos 
             GROUP BY combo ORDER BY cnt DESC LIMIT ?1",
        )?;

        let combos = stmt.query_map([limit as i64], |row| {
            Ok(ComboStats {
                combo: row.get(0)?,
                count: row.get(1)?,
            })
        })?;

        Ok(combos.filter_map(|c| c.ok()).collect())
    }

    fn get_key_count(&self, conn: &rusqlite::Connection, key_name: &str) -> Result<i64> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM key_events WHERE key_name = ?1",
            [key_name],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    fn get_hourly_distribution(&self, conn: &rusqlite::Connection) -> Result<Vec<HourlyStats>> {
        let mut stmt = conn.prepare(
            "SELECT hour, COUNT(*) as cnt FROM key_events 
             GROUP BY hour ORDER BY hour",
        )?;

        let hours = stmt.query_map([], |row| {
            Ok(HourlyStats {
                hour: row.get(0)?,
                count: row.get(1)?,
            })
        })?;

        // Fill in all 24 hours
        let mut hour_map: HashMap<i32, i64> = HashMap::new();
        for h in hours.filter_map(|h| h.ok()) {
            hour_map.insert(h.hour, h.count);
        }

        Ok((0..24)
            .map(|h| HourlyStats {
                hour: h,
                count: *hour_map.get(&h).unwrap_or(&0),
            })
            .collect())
    }

    fn get_daily_distribution(&self, conn: &rusqlite::Connection) -> Result<Vec<DailyStats>> {
        let mut stmt = conn.prepare(
            "SELECT day_of_week, COUNT(*) as cnt FROM key_events 
             GROUP BY day_of_week ORDER BY day_of_week",
        )?;

        let days = stmt.query_map([], |row| {
            let day_num: i32 = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((day_num, count))
        })?;

        let day_names = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        let mut day_map: HashMap<i32, i64> = HashMap::new();
        for d in days.filter_map(|d| d.ok()) {
            day_map.insert(d.0, d.1);
        }

        Ok((0..7)
            .map(|d| DailyStats {
                day: day_names[d as usize].to_string(),
                count: *day_map.get(&d).unwrap_or(&0),
            })
            .collect())
    }

    fn get_typing_speed_stats(&self, conn: &rusqlite::Connection) -> Result<(f64, f64)> {
        let avg: f64 = conn.query_row(
            "SELECT COALESCE(AVG(chars_per_minute), 0.0) FROM typing_samples",
            [],
            |row| row.get(0),
        )?;

        let max: f64 = conn.query_row(
            "SELECT COALESCE(MAX(chars_per_minute), 0.0) FROM typing_samples",
            [],
            |row| row.get(0),
        )?;

        Ok((avg, max))
    }

    fn get_key_frequency_map(&self, conn: &rusqlite::Connection) -> Result<HashMap<String, i64>> {
        let mut stmt =
            conn.prepare("SELECT key_name, COUNT(*) as cnt FROM key_events GROUP BY key_name")?;

        let keys = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        Ok(keys.filter_map(|k| k.ok()).collect())
    }

    fn get_first_recorded(&self, conn: &rusqlite::Connection) -> Result<Option<String>> {
        let result: Option<String> = conn
            .query_row(
                "SELECT timestamp FROM key_events ORDER BY timestamp ASC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();
        Ok(result)
    }

    fn get_last_recorded(&self, conn: &rusqlite::Connection) -> Result<Option<String>> {
        let result: Option<String> = conn
            .query_row(
                "SELECT timestamp FROM key_events ORDER BY timestamp DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();
        Ok(result)
    }
}
