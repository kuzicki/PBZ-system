use postgres::{Config, NoTls};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use super::dao::*;

pub type PostgrePool = Pool<PostgresConnectionManager<NoTls>>;

pub fn create_pool() -> PostgrePool {
    let mut config = Config::new();
    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let dbname = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    config
        .host("db")
        .user(&user)
        .password(&password)
        .dbname(&dbname);
    let manager = PostgresConnectionManager::new(config, NoTls);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create a pool")
}
