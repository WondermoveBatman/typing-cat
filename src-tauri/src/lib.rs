mod domain;
mod application;
mod infrastructure;
mod presentation;
mod state;

use state::AppState;
use infrastructure::tray::create_tray;
use infrastructure::persistence::Database;
use presentation::commands::{
    get_current_stats, get_daily_stats, get_weekly_stats, get_monthly_stats,
    start_tracking, stop_tracking, is_tracking, check_accessibility, save_stats,
};
use tauri::Manager;
use std::sync::{Arc, Mutex};
use domain::entities::TypingStats;
use log::{info, error};

/// Auto-save interval in seconds (5 minutes)
const AUTO_SAVE_INTERVAL_SECS: u64 = 300;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(AppState::new())
        .on_window_event(|window, event| {
            // Hide stats window when it loses focus (typical tray app behavior)
            if window.label() == "stats" {
                if let tauri::WindowEvent::Focused(false) = event {
                    let _ = window.hide();
                }
            }
        })
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");

            // Create directory if not exists
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            // Initialize database
            let db = Database::new(&app_data_dir)
                .expect("Failed to initialize database");

            // Update AppState with database and clone for auto-save timer
            let (stats_arc, db_arc) = {
                let state = app.state::<AppState>();
                *state.db.lock().unwrap() = Some(db);
                (state.stats.clone(), state.db.clone())
            };

            // Create system tray
            create_tray(app.handle())?;

            // macOS specific setup
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                use infrastructure::macos::{check_accessibility_permission, request_accessibility_permission};

                // Hide dock icon
                app.set_activation_policy(ActivationPolicy::Accessory);

                // Check accessibility permission on startup
                if !check_accessibility_permission() {
                    info!("Accessibility permission not granted, requesting...");
                    request_accessibility_permission();
                } else {
                    info!("Accessibility permission already granted");
                }
            }

            // Start auto-save background timer
            start_auto_save_timer(stats_arc, db_arc);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_current_stats,
            get_daily_stats,
            get_weekly_stats,
            get_monthly_stats,
            start_tracking,
            stop_tracking,
            is_tracking,
            check_accessibility,
            save_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Starts a background thread that auto-saves statistics periodically
///
/// The thread will save statistics every AUTO_SAVE_INTERVAL_SECS seconds.
/// Errors are logged but don't stop the timer.
fn start_auto_save_timer(stats: Arc<Mutex<TypingStats>>, db: Arc<Mutex<Option<Database>>>) {
    std::thread::spawn(move || {
        let save_interval = std::time::Duration::from_secs(AUTO_SAVE_INTERVAL_SECS);

        loop {
            std::thread::sleep(save_interval);

            match perform_auto_save(&stats, &db) {
                Ok(true) => info!("Auto-saved stats to database"),
                Ok(false) => {
                    // No data to save, continue silently
                }
                Err(e) => error!("Auto-save failed: {}", e),
            }
        }
    });
}

/// Performs auto-save of current statistics to the database
///
/// Returns Ok(true) if save was performed, Ok(false) if skipped (no data),
/// or Err with error message
fn perform_auto_save(
    stats: &Arc<Mutex<TypingStats>>,
    db: &Arc<Mutex<Option<Database>>>,
) -> Result<bool, String> {
    let stats_guard = stats.lock().map_err(|e| format!("Failed to lock stats: {}", e))?;
    let db_guard = db.lock().map_err(|e| format!("Failed to lock db: {}", e))?;

    // Skip if no data to save
    if stats_guard.total_keystrokes == 0 {
        return Ok(false);
    }

    // Skip if database is not initialized
    let database = match db_guard.as_ref() {
        Some(db) => db,
        None => return Err("Database not initialized".to_string()),
    };

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    database
        .update_daily_stats(
            &today,
            stats_guard.total_keystrokes as i64,
            stats_guard.printable_chars as i64,
        )
        .map_err(|e| format!("Database update failed: {}", e))?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::TypingStats;
    use crate::infrastructure::persistence::Database;

    #[test]
    fn test_perform_auto_save_with_no_data() {
        // Given: Empty statistics
        let stats = Arc::new(Mutex::new(TypingStats::new()));
        let db = Arc::new(Mutex::new(None::<Database>));

        // When: Attempting auto-save
        let result = perform_auto_save(&stats, &db);

        // Then: Should skip (return Ok(false))
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false, "Should skip when no keystrokes recorded");
    }

    #[test]
    fn test_perform_auto_save_without_database() {
        // Given: Stats with data but no database
        let stats = Arc::new(Mutex::new(TypingStats::new()));
        {
            let mut s = stats.lock().unwrap();
            s.record_keystroke(true);
            s.record_keystroke(false);
        }
        let db = Arc::new(Mutex::new(None::<Database>));

        // When: Attempting auto-save
        let result = perform_auto_save(&stats, &db);

        // Then: Should return error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Database not initialized"));
    }

    #[test]
    fn test_perform_auto_save_success() {
        // Given: Stats with data and initialized database
        let temp_dir = std::env::temp_dir().join("keystroke-counter-auto-save-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let stats = Arc::new(Mutex::new(TypingStats::new()));
        {
            let mut s = stats.lock().unwrap();
            s.record_keystroke(true);  // printable
            s.record_keystroke(true);  // printable
            s.record_keystroke(false); // non-printable
        }

        let database = Database::new(&temp_dir).unwrap();
        let db = Arc::new(Mutex::new(Some(database)));

        // When: Performing auto-save
        let result = perform_auto_save(&stats, &db);

        // Then: Should succeed
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true, "Should save when data exists");

        // Verify data was saved to database
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let db_guard = db.lock().unwrap();
        let saved_stats = db_guard.as_ref().unwrap().get_daily_stats(&today).unwrap();

        assert!(saved_stats.is_some());
        let saved_stats = saved_stats.unwrap();
        assert_eq!(saved_stats.total_keystrokes, 3);
        assert_eq!(saved_stats.printable_chars, 2);

        // Clean up
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
