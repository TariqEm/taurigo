mod commands;
mod db;
mod menu;
mod services;
mod sidecar;
mod state;
mod tray;

use tauri::Manager;

use state::{AppConfig, AppState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // reqwest (used for the sidecar health round-trip in `services::sidecar`, and
    // transitively by `tauri-plugin-updater`) is built against rustls, which since
    // 0.23 requires a crypto provider installed process-wide before any client can
    // be built — even for a plain loopback `http://` request. `.ok()`: an `Err`
    // just means something else installed one first, which is fine.
    let _ = rustls::crypto::ring::default_provider().install_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // DB pool + migrations (6.3/6.5). Real migration files land in Phase 10 —
            // `migrations/` is empty today, so this only proves the plumbing works.
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("taurigo.sqlite3");

            let db_pool = db::create_pool(&db_path)?;
            db::run_migrations(&db_pool)?;

            let config = AppConfig { data_dir, db_path };
            app.manage(AppState::new(db_pool, config));

            // Native chrome (6.6).
            let app_menu = menu::build(app.handle())?;
            app.set_menu(app_menu)?;
            tray::build(app.handle())?;

            // Go sidecar (7.5): spawn + supervise for the lifetime of the app. Reads
            // its `PORT=` handshake and stores the base URL on `AppState`; restarts
            // it (bounded) if it exits unexpectedly.
            sidecar::spawn_supervised(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::ping,
            commands::sidecar::sidecar_health
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // The sidecar is spawned directly from Rust rather than via
            // `tauri-plugin-shell`'s JS-facing `spawn` command, so it isn't in the
            // plugin's own child registry that it auto-kills on exit — kill it
            // ourselves so it doesn't linger as an orphan process.
            if let tauri::RunEvent::Exit = event {
                sidecar::kill_on_exit(app_handle);
            }
        });
}
