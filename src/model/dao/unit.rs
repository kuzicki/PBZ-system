use super::{tech::Tech, Error, PostgrePool, Row};
use std::rc::Rc;

#[derive(Debug)]
pub struct Unit {
    id: Option<i32>,
    pub unit_number: i32,
    pub full_name: String,
    pub short_name: String,
}

impl Unit {
    fn from_row(row: Row) -> Result<Unit, Error> {
        Ok(Unit {
            id: row.get(0),
            unit_number: row.get(1),
            full_name: row.get(2),
            short_name: row.get(3),
        })
    }

    fn from_row_with_name(row: Row) -> Result<(String, Unit), Error> {
        Ok((row.get(0),  Unit {
            id: row.get(0),
            unit_number: row.get(1),
            full_name: row.get(2),
            short_name: row.get(3),
        }))
    }

    pub fn new(unit_number: i32, full_name: &str, short_name: &str) -> Unit {
        Unit {
            id: None,
            unit_number,
            full_name: full_name.into(),
            short_name: short_name.into(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(-1)
    } 

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

pub struct UnitDAOImpl {
    pool: Rc<PostgrePool>,
}

impl UnitDAOImpl {
    pub fn new(pool: Rc<PostgrePool>) -> Self {
        Self { pool }
    }

    pub fn insert(&self, unit: &mut Unit) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();

        let id: i32 = conn
            .query_one(
                Self::INSERT,
                &[
                    &unit.unit_number,
                    &unit.full_name,
                    &unit.short_name,
                ],
            )
            .map(|row| row.get(0))?;

        unit.id = Some(id);
        Ok(())
    }

    pub fn update(&self, unit: &Unit) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(
            Self::UPDATE,
            &[
                &unit.unit_number,
                &unit.full_name,
                &unit.short_name,
                &unit.id
            ],
        )?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Unit>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_unit = vec![];
        for row in conn.query(Self::FIND_ALL, &[])? {
            all_unit.push(Unit::from_row(row)?);
        }
        Ok(all_unit)
    }

    pub fn get_all_acountable(&self) -> Result<Vec<Unit>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_unit = vec![];
        for row in conn.query(Self::FIND_ALL_ACCOUNTABLE, &[])? {
            all_unit.push(Unit::from_row(row)?);
        }
        Ok(all_unit)
    }

    pub fn get_id(&self, id: i32) -> Result<Unit, Error> {
        let mut conn = self.pool.get().unwrap();
        let row = conn.query_one(Self::FIND_ID, &[&id])?;
        Ok(Unit::from_row(row)?)
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(Self::DELETE, &[&id])?;
        Ok(())
    }


    const INSERT: &'static str =
        "INSERT INTO unit (unit_number, full_name, short_name) VALUES ($1, $2, $3) RETURNING id";
    const UPDATE: &'static str =
        "UPDATE unit SET unit_number = $1, full_name = $2, short_name = $3 WHERE id = $4";
    const FIND_ALL: &'static str = "SELECT id, unit_number, full_name, short_name FROM unit";
    const FIND_ID: &'static str =
        "SELECT id, unit_number, full_name, short_name FROM unit WHERE id = $1";

    const DELETE: &'static str = "DELETE FROM unit WHERE id = $1";
    const TECH_LIST: &'static str = "";
    const FIND_ALL_ACCOUNTABLE: &'static str = r#"SELECT DISTINCT u.id, u.unit_number, u.full_name, u.short_name
    FROM unit u
    JOIN employee e ON u.id = e.unit_id
    WHERE e.is_accountable = true;"#;
}
