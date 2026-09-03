//! SQLite connection pool + migration runner.
//!
//! This crate standardizes on `rusqlite` (via `r2d2`/`r2d2_sqlite` for pooling) as the
//! sole DB access path — see `Cargo.toml` and `BUILD_TIMELINE.md` Phase 6.2 for why
//! `tauri-plugin-sql` (sqlx-based) was dropped instead.
//!
//! Actual schema/FTS5/`sqlite-vec` migration files are Phase 10's job. Right now
//! `migrations/` is empty (just `.gitkeep`), so `run_migrations` only proves the
//! plumbing — pool -> bookkeeping table -> directory scan -> apply-in-order — works
//! cleanly on startup with nothing to apply yet.

use std::path::{Path, PathBuf};

use r2d2_sqlite::SqliteConnectionManager;

/// Pooled SQLite connections, shared via `AppState`.
pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("failed to build the sqlite connection pool: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("failed to read `{0}`: {1}")]
    Io(PathBuf, std::io::Error),
}

/// Directory of versioned `.sql` migration files, applied in filename order.
/// Mirrors `packages/db-schema` (source of truth) — see `CLAUDE.md`.
fn migrations_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

/// Build a pooled connection manager for the sqlite database at `db_path`, with
/// sane defaults (WAL journaling, foreign keys on) applied to every pooled connection.
pub fn create_pool(db_path: &Path) -> Result<DbPool, DbError> {
    let manager = SqliteConnectionManager::file(db_path).with_init(|conn| {
        conn.pragma_update(None, "foreign_keys", true)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        Ok(())
    });
    Ok(r2d2::Pool::builder().build(manager)?)
}

/// Ensure the `_migrations` bookkeeping table exists, then apply any `.sql` files
/// under `migrations/` that haven't run yet, in filename order, inside the calling
/// pool's default connection.
pub fn run_migrations(pool: &DbPool) -> Result<(), DbError> {
    let conn = pool.get()?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            name TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    let dir = migrations_dir();
    if !dir.is_dir() {
        return Ok(());
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| DbError::Io(dir.clone(), e))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("sql"))
        .collect();
    files.sort();

    for path in files {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let already_applied: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?1)",
            [&name],
            |row| row.get(0),
        )?;
        if already_applied {
            continue;
        }

        let sql = std::fs::read_to_string(&path).map_err(|e| DbError::Io(path.clone(), e))?;
        conn.execute_batch(&sql)?;
        conn.execute("INSERT INTO _migrations (name) VALUES (?1)", [&name])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_migrations_is_idempotent_against_an_empty_migrations_dir() {
        let pool = create_pool(Path::new(":memory:")).expect("pool should build");
        run_migrations(&pool).expect("first run should succeed");
        run_migrations(&pool).expect("second run should be a no-op, not an error");
    }
}
