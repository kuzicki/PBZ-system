use postgres::{Client, Error, NoTls, Row};

pub mod dao {
    use super::pool::PostgrePool;
    use super::{Error, Row};

    pub mod employee;
    // pub mod room;
    pub mod tech;
    pub mod transfer;
    pub mod unit;
    pub mod room;
}

pub mod pool;
