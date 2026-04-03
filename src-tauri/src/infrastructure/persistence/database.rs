use rusqlite::{Connection, Result, params};
use std::path::Path;
use log::info;

use crate::domain::entities::DailyStats;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_data_dir: &Path) -> Result<Self> {
        let db_path = app_data_dir.join("typing_stats.db");
        info!("Opening database at: {:?}", db_path);

        let conn = Connection::open(&db_path)?;

        // Run migrations
        conn.execute_batch(include_str!("../../../migrations/001_initial.sql"))?;

        Ok(Self { conn })
    }

    pub fn update_daily_stats(
        &self,
        date: &str,
        keystrokes: i64,
        chars: i64,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO daily_stats (date, total_keystrokes, printable_chars)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(date) DO UPDATE SET
                total_keystrokes = total_keystrokes + ?2,
                printable_chars = printable_chars + ?3,
                updated_at = CURRENT_TIMESTAMP",
            params![date, keystrokes, chars],
        )?;
        Ok(())
    }

    pub fn get_daily_stats(&self, date: &str) -> Result<Option<DailyStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT date, total_keystrokes, printable_chars, typing_duration_seconds, session_count
             FROM daily_stats WHERE date = ?1"
        )?;

        let mut rows = stmt.query(params![date])?;

        if let Some(row) = rows.next()? {
            Ok(Some(DailyStats {
                date: row.get(0)?,
                total_keystrokes: row.get::<_, i64>(1)? as u64,
                printable_chars: row.get::<_, i64>(2)? as u64,
                typing_duration_seconds: row.get::<_, i64>(3)? as u64,
                session_count: row.get::<_, i32>(4)? as u32,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_stats_range(&self, start: &str, end: &str) -> Result<Vec<DailyStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT date, total_keystrokes, printable_chars, typing_duration_seconds, session_count
             FROM daily_stats
             WHERE date BETWEEN ?1 AND ?2
             ORDER BY date"
        )?;

        let rows = stmt.query_map(params![start, end], |row| {
            Ok(DailyStats {
                date: row.get(0)?,
                total_keystrokes: row.get::<_, i64>(1)? as u64,
                printable_chars: row.get::<_, i64>(2)? as u64,
                typing_duration_seconds: row.get::<_, i64>(3)? as u64,
                session_count: row.get::<_, i32>(4)? as u32,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }
}
