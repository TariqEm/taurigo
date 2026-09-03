//! Native menu bar.
//!
//! A modest starter menu (File/Edit/Window) — richer, app-specific menu items
//! (command palette actions, recent files, etc.) land alongside their features in
//! later phases (see `BUILD_TIMELINE.md` Phase 11).

use tauri::menu::{Menu, MenuBuilder, PredefinedMenuItem, SubmenuBuilder};
use tauri::{AppHandle, Runtime};

pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&PredefinedMenuItem::close_window(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::quit(app, None)?)
        .build()?;

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let window_menu = SubmenuBuilder::new(app, "Window")
        .minimize()
        .fullscreen()
        .build()?;

    MenuBuilder::new(app)
        .item(&file_menu)
        .item(&edit_menu)
        .item(&window_menu)
        .build()
}
