use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
    menu::{MenuBuilder, MenuItemBuilder},
    Manager, Runtime, AppHandle, Emitter,
    WebviewWindowBuilder, WebviewUrl,
    image::Image,
};
use tauri_plugin_positioner::{Position, WindowExt};
use log::info;
use chrono;

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    info!("Creating system tray");

    // Create menu items
    let show_stats = MenuItemBuilder::new("통계 보기")
        .id("show_stats")
        .build(app)?;

    let start_tracking = MenuItemBuilder::new("추적 시작")
        .id("start_tracking")
        .build(app)?;

    let stop_tracking = MenuItemBuilder::new("추적 중지")
        .id("stop_tracking")
        .build(app)?;

    let quit = MenuItemBuilder::new("종료")
        .id("quit")
        .build(app)?;

    // Build menu
    let menu = MenuBuilder::new(app)
        .items(&[&show_stats, &start_tracking, &stop_tracking, &quit])
        .build()?;

    // Load tray icon (using template icon for macOS menu bar)
    // Template icons should be black/white PNG for proper dark/light mode support
    let icon = Image::from_bytes(include_bytes!("../../../icons/32x32.png"))
        .expect("Failed to load tray icon");

    // Build tray icon
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .menu_on_left_click(false)  // Left click shows window, right click shows menu
        .tooltip("Keystroke Counter")
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "show_stats" => {
                    info!("Show stats clicked");
                    show_stats_window(app);
                }
                "start_tracking" => {
                    info!("Start tracking clicked");
                    let _ = app.emit("start-tracking", ());
                }
                "stop_tracking" => {
                    info!("Stop tracking clicked");
                    let _ = app.emit("stop-tracking", ());
                }
                "quit" => {
                    info!("Quit clicked - saving stats before exit");

                    // Save stats before exit
                    if let Some(state) = app.try_state::<crate::state::AppState>() {
                        let stats = state.stats.lock().unwrap();
                        let db_guard = state.db.lock().unwrap();

                        if let Some(db) = db_guard.as_ref() {
                            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                            if let Err(e) = db.update_daily_stats(
                                &today,
                                stats.total_keystrokes as i64,
                                stats.printable_chars as i64,
                            ) {
                                log::error!("Failed to save stats on quit: {}", e);
                            } else {
                                info!("Stats saved successfully before exit");
                            }
                        }
                    }

                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: MouseButton::Left, .. } => {
                    info!("Tray icon left clicked");
                    let app = tray.app_handle();
                    show_stats_window(app);
                }
                _ => {}
            }
        })
        .build(app)?;

    info!("System tray created successfully");
    Ok(())
}

fn show_stats_window<R: Runtime>(app: &AppHandle<R>) {
    // Check if window already exists
    if let Some(window) = app.get_webview_window("stats") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            // Position near tray icon
            let _ = window.as_ref().window().move_window(Position::TrayCenter);
        }
        return;
    }

    // Create new window
    if let Ok(window) = WebviewWindowBuilder::new(
        app,
        "stats",
        WebviewUrl::App("index.html".into()),
    )
    .title("Keystroke Counter")
    .inner_size(320.0, 400.0)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(true)
    .build()
    {
        // Position near tray icon
        let _ = window.as_ref().window().move_window(Position::TrayCenter);
    }
}
