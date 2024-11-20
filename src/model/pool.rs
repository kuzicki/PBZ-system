use postgres::{Config, NoTls};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use super::dao::*;

pub type PostgrePool = Pool<PostgresConnectionManager<NoTls>>;

pub fn create_pool() -> PostgrePool {
    let mut config = Config::new();
    config
        .host("localhost")
        .user("postgres")
        .password("123")
        .dbname("PBZ2");
    let manager = PostgresConnectionManager::new(config, NoTls);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create a pool")
}
