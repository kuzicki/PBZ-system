use super::{tech::Tech, Error, PostgrePool, Row};
use std::rc::Rc;

#[derive(Debug)]
pub struct Employee {
    id: Option<i32>,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub is_supervisor: bool,
    pub is_accountable: bool,
    pub job_title: String,
    pub unit_id: i32,
}

impl Employee {
    fn from_row(row: Row) -> Result<Employee, Error> {
        Ok(Employee {
            id: row.get(0),
            first_name: row.get(1),
            middle_name: row.get(2),
            last_name: row.get(3),
            is_supervisor: row.get(4),
            is_accountable: row.get(5),
            job_title: row.get(6),
            unit_id: row.get(7),
        })
    }

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id)
    }

    pub fn new(
        first_name: &str,
        middle_name: &str,
        last_name: &str,
        is_supervisor: bool,
        is_accountable: bool,
        job_title: &str,
        unit_id: i32,
    ) -> Employee {
        Employee {
            id: None,
            first_name: first_name.into(),
            middle_name: middle_name.into(),
            last_name: last_name.into(),
            is_supervisor,
            is_accountable,
            job_title: job_title.into(),
            unit_id,
        }
    }
}

pub struct EmployeeDAOImpl {
    pool: Rc<PostgrePool>,
}

impl EmployeeDAOImpl {
    pub fn new(pool: Rc<PostgrePool>) -> Self {
        Self { pool }
    }

    pub fn insert(&self, employee: &mut Employee) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();

        let id: i32 = conn
            .query_one(
                Self::INSERT,
                &[
                    &employee.first_name,
                    &employee.middle_name,
                    &employee.last_name,
                    &employee.is_supervisor,
                    &employee.is_accountable,
                    &employee.job_title,
                    &employee.unit_id,
                ],
            )
            .map(|row| row.get(0))?;
        employee.id = Some(id);

        Ok(())
    }

    pub fn update(&self, employee: &Employee) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(
            Self::UPDATE,
            &[
                &employee.first_name,
                &employee.middle_name,
                &employee.last_name,
                &employee.is_supervisor,
                &employee.is_accountable,
                &employee.job_title,
                &employee.unit_id,
                &employee.id,
            ],
        )?;

        Ok(())
    }

    pub fn get_all_accountable(&self, unit_id: i32) -> Result<Vec<Employee>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_employees = vec![];

        for row in conn.query(Self::FIND_ALL_ACCOUNTABLE_UNIT, &[&unit_id])? {
            all_employees.push(Employee::from_row(row)?);
        }
        Ok(all_employees)
    }

    pub fn get_all(&self) -> Result<Vec<Employee>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_employee = vec![];
        for row in conn.query(Self::FIND_ALL, &[])? {
            all_employee.push(Employee::from_row(row)?);
        }
        Ok(all_employee)
    }

    pub fn get_id(&self, id: i32) -> Result<Employee, Error> {
        let mut conn = self.pool.get().unwrap();
        let row = conn.query_one(Self::FIND_ID, &[&id])?;
        Ok(Employee::from_row(row)?)
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(Self::DELETE, &[&id])?;
        Ok(())
    }

    const INSERT: &'static str = "INSERT INTO employee (first_name, middle_name, last_name, is_supervisor, is_accountable, job_title, unit_id)
VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id;";
    const UPDATE: &'static str = "UPDATE employee SET first_name = $1, middle_name = $2, last_name = $3, is_supervisor = $4, is_accountable = $5, job_title = $6, unit_id = $7 WHERE id = $8;";
    const DELETE: &'static str = "DELETE FROM employee WHERE id = $1;";
    const FIND_ALL: &'static str = "SELECT * FROM employee;";
    const FIND_ID: &'static str = "SELECT * FROM employee WHERE id = $1;";
    const FIND_ALL_ACCOUNTABLE_UNIT: &'static str =
        "SELECT * FROM employee WHERE unit_id = $1 AND is_accountable = TRUE;";
}
