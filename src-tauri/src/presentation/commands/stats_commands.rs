use tauri::State;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use log::info;

use crate::state::AppState;
use crate::domain::entities::DailyStats;
use crate::infrastructure::keyboard::start_keyboard_listener;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentStats {
    pub total_keystrokes: u64,
    pub printable_chars: u64,
    pub wpm: f64,
    pub cpm: f64,
    pub is_session_active: bool,
}

#[tauri::command]
pub fn get_current_stats(state: State<AppState>) -> CurrentStats {
    let stats = state.stats.lock().unwrap();

    CurrentStats {
        total_keystrokes: stats.total_keystrokes,
        printable_chars: stats.printable_chars,
        wpm: stats.calculate_wpm(),
        cpm: stats.calculate_cpm(),
        is_session_active: stats.is_session_active(),
    }
}

#[tauri::command]
pub fn get_daily_stats(state: State<AppState>, date: Option<String>) -> Option<DailyStats> {
    let date = date.unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let db_guard = state.db.lock().unwrap();
    if let Some(db) = db_guard.as_ref() {
        db.get_daily_stats(&date).ok().flatten()
    } else {
        None
    }
}

#[tauri::command]
pub fn get_weekly_stats(state: State<AppState>) -> Vec<DailyStats> {
    let end = Utc::now();
    let start = end - Duration::days(7);

    let start_str = start.format("%Y-%m-%d").to_string();
    let end_str = end.format("%Y-%m-%d").to_string();

    let db_guard = state.db.lock().unwrap();
    if let Some(db) = db_guard.as_ref() {
        db.get_stats_range(&start_str, &end_str).unwrap_or_default()
    } else {
        Vec::new()
    }
}

#[tauri::command]
pub fn get_monthly_stats(state: State<AppState>) -> Vec<DailyStats> {
    let end = Utc::now();
    let start = end - Duration::days(30);

    let start_str = start.format("%Y-%m-%d").to_string();
    let end_str = end.format("%Y-%m-%d").to_string();

    let db_guard = state.db.lock().unwrap();
    if let Some(db) = db_guard.as_ref() {
        db.get_stats_range(&start_str, &end_str).unwrap_or_default()
    } else {
        Vec::new()
    }
}

#[tauri::command]
pub fn start_tracking(state: State<AppState>) -> bool {
    info!("Starting keystroke tracking");

    let mut tracking = state.is_tracking.lock().unwrap();
    if *tracking {
        return true; // Already tracking
    }

    *tracking = true;

    // Start keyboard listener
    let stats = state.stats.clone();
    let is_tracking = state.is_tracking.clone();
    let _rx = start_keyboard_listener(stats, is_tracking);

    true
}

#[tauri::command]
pub fn stop_tracking(state: State<AppState>) -> bool {
    info!("Stopping keystroke tracking");

    let mut tracking = state.is_tracking.lock().unwrap();
    *tracking = false;

    true
}

#[tauri::command]
pub fn is_tracking(state: State<AppState>) -> bool {
    let tracking = state.is_tracking.lock().unwrap();
    *tracking
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityStatus {
    pub granted: bool,
    pub platform: String,
}

#[tauri::command]
pub fn save_stats(state: State<AppState>) -> Result<(), String> {
    let stats = state.stats.lock().unwrap();
    let db_guard = state.db.lock().unwrap();

    if let Some(db) = db_guard.as_ref() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        db.update_daily_stats(
            &today,
            stats.total_keystrokes as i64,
            stats.printable_chars as i64,
        ).map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub fn check_accessibility() -> AccessibilityStatus {
    #[cfg(target_os = "macos")]
    {
        use crate::infrastructure::macos::{check_accessibility_permission, request_accessibility_permission};

        let granted = check_accessibility_permission();
        if !granted {
            // Show system permission dialog
            request_accessibility_permission();
        }

        AccessibilityStatus {
            granted,
            platform: "macos".to_string(),
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        AccessibilityStatus {
            granted: true, // Other platforms don't need special permission
            platform: "other".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::Database;

    #[test]
    fn test_save_stats_logic() {
        // Test the save_stats logic by directly testing database integration
        let temp_dir = std::env::temp_dir().join("keystroke-counter-test-save");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).expect("Failed to create test dir");

        let db = Database::new(&temp_dir).expect("Failed to create database");
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Test saving stats
        let result = db.update_daily_stats(&today, 100, 50);
        assert!(result.is_ok(), "Should be able to save stats");

        // Verify data was saved
        let saved_stats = db.get_daily_stats(&today);
        assert!(saved_stats.is_ok(), "Should be able to retrieve stats");

        let saved = saved_stats.unwrap();
        assert!(saved.is_some(), "Stats should exist");

        let saved = saved.unwrap();
        assert_eq!(saved.total_keystrokes, 100, "Keystrokes should match");
        assert_eq!(saved.printable_chars, 50, "Chars should match");

        std::fs::remove_dir_all(&temp_dir).expect("Failed to clean up");
    }

    #[test]
    fn test_save_stats_incremental() {
        // Test that stats accumulate correctly
        let temp_dir = std::env::temp_dir().join("keystroke-counter-test-incremental");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).expect("Failed to create test dir");

        let db = Database::new(&temp_dir).expect("Failed to create database");
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Save first batch
        db.update_daily_stats(&today, 100, 50).expect("First save failed");

        // Save second batch
        db.update_daily_stats(&today, 50, 25).expect("Second save failed");

        // Verify accumulated stats
        let saved = db.get_daily_stats(&today).unwrap().unwrap();
        assert_eq!(saved.total_keystrokes, 150, "Keystrokes should accumulate");
        assert_eq!(saved.printable_chars, 75, "Chars should accumulate");

        std::fs::remove_dir_all(&temp_dir).expect("Failed to clean up");
    }
}
