use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;

pub fn create_pool(db_path: &str) -> SqlitePool {
    let manager = SqliteConnectionManager::file(db_path);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create a pool")
}
