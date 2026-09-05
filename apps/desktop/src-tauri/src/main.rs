// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// `app_lib::run()` below is genuinely unreachable when `export-bindings` is enabled
// (the cfg'd block always `return`s first) — that's intentional: this feature only
// exists to run the headless export instead of the GUI, never both.
#[cfg_attr(feature = "export-bindings", allow(unreachable_code))]
fn main() {
    #[cfg(feature = "export-bindings")]
    {
        app_lib::specta_builder()
            .export(
                specta_typescript::Typescript::default(),
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../../packages/types/src/bindings.ts"
                ),
            )
            .expect("failed to export typescript bindings");
        return;
    }

    app_lib::run();
}
