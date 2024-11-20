use rusqlite::{params, Connection};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;

fn create_pool(db_path: &str) -> SqlitePool {
    let manager = SqliteConnectionManager::file(db_path);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct User {
    id: i32,
    name: String,
}

pub struct UserDao {
    pool: SqlitePool,
}

impl UserDao {
    pub fn new(pool: SqlitePool) -> Self {
        UserDao { pool }
    }

    pub fn create(&self, name: &str) -> rusqlite::Result<()> {
        let conn: PooledConnection<SqliteConnectionManager> = self.pool.get()?;
        conn.execute("INSERT INTO users (name) VALUES (?1)", params![name])?;
        Ok(())
    }

    pub fn read(&self, id: i32) -> rusqlite::Result<User> {
        let conn: PooledConnection<SqliteConnectionManager> = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, name FROM users WHERE id = ?1")?;
        let user = stmt.query_row(params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        Ok(user)
    }

    // Other methods (update, delete) would be similar...
}

// Example of another DAO
pub struct Product {
    id: i32,
    name: String,
}

pub struct ProductDao {
    pool: SqlitePool,
}

impl ProductDao {
    pub fn new(pool: SqlitePool) -> Self {
        ProductDao { pool }
    }

    pub fn create(&self, name: &str) -> rusqlite::Result<()> {
        let conn: PooledConnection<SqliteConnectionManager> = self.pool.get()?;
        conn.execute("INSERT INTO products (name) VALUES (?1)", params![name])?;
        Ok(())
    }

    // Other methods would follow a similar pattern...
}

fn main() -> rusqlite::Result<()> {
    let pool = create_pool("my_database.db");

    let user_dao = UserDao::new(pool.clone());
    user_dao.create("Alice")?;

    let product_dao = ProductDao::new(pool);
    product_dao.create("Widget")?;

    // Use other DAOs as needed...

    Ok(())
}
