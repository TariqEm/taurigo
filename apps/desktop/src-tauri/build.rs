fn main() {
    // Forward Cargo's `TARGET` triple as a compile-time env var so
    // `services::sidecar`'s integration test can locate the Go sidecar binary
    // `apps/sidecar`'s `bun run build` produces
    // (`binaries/sidecar-<target-triple>`) without re-deriving the triple itself.
    println!(
        "cargo:rustc-env=TARGET_TRIPLE={}",
        std::env::var("TARGET").unwrap_or_default()
    );
    tauri_build::build()
}
