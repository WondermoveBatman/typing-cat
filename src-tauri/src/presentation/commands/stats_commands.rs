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
