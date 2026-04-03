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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(AppState::new())
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

            // Update AppState with database
            let state = app.state::<AppState>();
            *state.db.lock().unwrap() = Some(db);

            // Create system tray
            create_tray(app.handle())?;

            // macOS specific setup
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                use infrastructure::macos::{check_accessibility_permission, request_accessibility_permission};
                use log::info;

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
