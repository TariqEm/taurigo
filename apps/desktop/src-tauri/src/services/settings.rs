//! Settings-domain business logic backing `commands::settings`.

use crate::db::DbPool;

/// Returns `true` if a pooled connection can be acquired and a trivial round-trip
/// query succeeds.
pub fn check_db_connection(pool: &DbPool) -> bool {
    let Ok(conn) = pool.get() else {
        return false;
    };
    conn.query_row("SELECT 1", [], |row| row.get::<_, i64>(0))
        .map(|value| value == 1)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;
    use std::path::Path;

    #[test]
    fn check_db_connection_succeeds_against_an_in_memory_pool() {
        let pool = create_pool(Path::new(":memory:")).expect("pool should build");
        assert!(check_db_connection(&pool));
    }
}
