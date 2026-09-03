mod commands;
mod db;
mod menu;
mod services;
mod state;
mod tray;

use tauri::Manager;

use state::{AppConfig, AppState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::settings::ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
