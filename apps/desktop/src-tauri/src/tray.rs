//! System tray icon.
//!
//! Minimal starter tray: left-click shows/focuses the main window, right-click
//! (native default) opens a menu with a single Quit item. Richer tray behavior
//! (status indicators, quick actions) is a later-phase feature.

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_item])?;

    let icon = app
        .default_window_icon()
        .cloned()
        .expect("bundle.icon should provide a default window icon");

    TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .icon(icon)
        .tooltip("Taurigo")
        .on_menu_event(|app, event| {
            if event.id() == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
