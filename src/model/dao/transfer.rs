use super::{employee::Employee, room::Room, tech::Tech, unit::Unit};
use super::{Error, PostgrePool, Row};
use chrono::offset::Utc;
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub struct Transfer {
    id: Option<i32>,
    pub transfer_date: NaiveDate,
    pub tech_id: i32,
    pub room_id: i32,
    pub employee_id: i32,
}

#[derive(Debug)]
pub struct TransferUnit {
    pub inner: Transfer,
    pub unit_id: i32,
}

#[derive(Debug)]
pub struct TransferDetails {
    pub id: i32,
    pub transfer_date: NaiveDate,
    pub tech: Option<Tech>,
    pub room: Option<Room>,
    pub employee: Option<Employee>,
    pub unit: Option<Unit>,
}

impl TransferDetails {}

impl Transfer {
    fn from_row(row: Row) -> Result<Transfer, Error> {
        Ok(Transfer {
            id: row.get(0),
            transfer_date: row.get(1),
            tech_id: row.try_get(2).unwrap_or(0),
            room_id: row.try_get(3).unwrap_or(0),
            employee_id: row.try_get(4).unwrap_or(0),
        })
    }

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }
    
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id)
    }

    pub fn new(transfer_date: &str, tech_id: i32, room_id: i32, employee_id: i32) -> Transfer {
        Transfer {
            id: None,
            transfer_date: transfer_date.parse().expect("Invalid date"),
            tech_id,
            room_id,
            employee_id,
        }
    }

    pub fn new_date(
        transfer_date: NaiveDate,
        tech_id: i32,
        room_id: i32,
        employee_id: i32,
    ) -> Transfer {
        Transfer {
            id: None,
            transfer_date,
            tech_id,
            room_id,
            employee_id,
        }
    }
}

pub struct TransferDAOImpl {
    pool: Rc<PostgrePool>,
}

impl TransferDAOImpl {
    pub fn new(pool: Rc<PostgrePool>) -> Self {
        Self { pool }
    }

    pub fn get_detailed_transfers(
        &self,
    ) -> Result<Vec<TransferDetails>, Box<dyn std::error::Error>> {
        let mut transfers = Vec::new();
        let mut conn = self.pool.get().unwrap();

        for row in conn.query(Self::FIND_ALL_DETAILED, &[])? {
            let transfer = TransferDetails {
                id: row.get("transfer_id"),
                transfer_date: row.get::<_, NaiveDate>("transfer_date"),
                tech: row.try_get("tech_id").ok().and_then(|id: i32| {
                    let date = row.get::<_, NaiveDate>("acquisition_date");
                    let mut tech = Tech::new_with_date(
                        row.get("inventory_number"),
                        row.get("tech_name"),
                        row.get("model"),
                        date,
                        row.get("price"),
                    );
                    tech.set_id(id);
                    Some(tech)
                }),
                room: row.try_get("room_id").ok().map(|id: i32| {
                    let mut room = Room::new(row.get("room_number"), row.get("squares"));
                    room.set_id(id);
                    room
                }),
                employee: row.try_get("employee_id").ok().map(|id: i32| {
                    let mut employee = Employee::new(
                        row.get("first_name"),
                        row.get("middle_name"),
                        row.get("last_name"),
                        row.get("is_supervisor"),
                        row.get("is_accountable"),
                        row.get("job_title"),
                        row.get("unit_id"),
                    );
                    employee.set_id(id);
                    employee
                }),
                unit: row.try_get("unit_id").ok().map(|id: i32| {
                    let mut unit = Unit::new(
                        row.get("unit_number"),
                        row.get("full_name"),
                        row.get("short_name"),
                    );
                    unit.set_id(id);
                    unit
                }),
            };
            transfers.push(transfer);
        }
        Ok(transfers)
    }

    pub fn insert(&self, transfer: &mut Transfer) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();

        let id: i32 = conn
            .query_one(
                Self::INSERT,
                &[
                    &transfer.transfer_date,
                    &transfer.tech_id,
                    &transfer.room_id,
                    &transfer.employee_id,
                ],
            )
            .map(|row| row.get(0))?;

        transfer.id = Some(id);

        Ok(())
    }

    pub fn update(&self, transfer: &Transfer) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(
            Self::UPDATE,
            &[
                &transfer.transfer_date,
                &transfer.tech_id,
                &transfer.room_id,
                &transfer.employee_id,
                &transfer.id,
            ],
        )?;
        println!("Affected: {}", rows_affected);

        Ok(())
    }

    fn get_all(&self) -> Result<Vec<Transfer>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_transfers = vec![];
        for row in conn.query(Self::FIND_ALL, &[])? {
            all_transfers.push(Transfer::from_row(row)?);
        }
        Ok(all_transfers)
    }

    fn get_id(&self, id: i32) -> Result<Transfer, Error> {
        let mut conn = self.pool.get().unwrap();
        let row = conn.query_one(Self::FIND_ID, &[&id])?;
        Ok(Transfer::from_row(row)?)
    }

    pub fn get_id_with_unit(&self, id: i32) -> Result<TransferUnit, Error> {
        let mut conn = self.pool.get().unwrap();
        let row = conn.query_one(Self::FIND_ID_UNIT, &[&id])?;
        let unit_id = row.try_get(5).unwrap_or(0);
        let transfer = Transfer::from_row(row)?;
        Ok(TransferUnit {
            inner: transfer,
            unit_id,
        })
    }

    const INSERT: &'static str = "INSERT INTO transfer (transfer_date, tech_id, room_id, employee_id) VALUES ($1, $2, $3, $4) RETURNING id;";
    const UPDATE: &'static str = "UPDATE transfer SET transfer_date = $1, tech_id = $2, room_id = $3, employee_id = $4 WHERE id = $5;";
    const FIND_ALL: &'static str = "SELECT * FROM transfer;";
    const FIND_ID: &'static str = "SELECT * FROM transfer WHERE id = $1;";
    const FIND_ID_UNIT: &'static str = r#"SELECT transfer.*, employee.unit_id FROM transfer 
LEFT JOIN employee ON transfer.employee_id = employee.id
WHERE transfer.id = $1;"#;
    const FIND_ALL_DETAILED: &'static str = r#"SELECT 
        transfer.id AS transfer_id,
        transfer.transfer_date,
        tech.id AS tech_id,
        tech.inventory_number,
        tech.name AS tech_name,
        tech.model,
        tech.acquisition_date,
        tech.price,
        room.id AS room_id,
        room.room_number,
        room.squares,
        employee.id AS employee_id,
        employee.first_name,
        employee.middle_name,
        employee.last_name,
        employee.is_supervisor,
        employee.is_accountable,
        employee.job_title,
        employee.unit_id,
        unit.id AS unit_id,
        unit.unit_number,
        unit.full_name,
        unit.short_name
    FROM 
        transfer
    LEFT JOIN tech ON transfer.tech_id = tech.id
        LEFT JOIN room ON transfer.room_id = room.id
        LEFT JOIN employee ON transfer.employee_id = employee.id
        LEFT JOIN unit ON employee.unit_id = unit.id;"#;
}
