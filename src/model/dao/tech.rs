use super::{Error, PostgrePool, Row};
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub struct Tech {
    id: Option<i32>,
    pub inventory_number: i32,
    pub name: String,
    pub model: String,
    pub acquisition_date: NaiveDate,
    pub price: i32,
}

#[derive(Debug)]
pub struct TechUnit {
    pub unit_name: String,
    pub date: NaiveDate,
    pub inventory_number: i32,
    pub name: String,
    pub model: String,
}

impl TechUnit {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            unit_name: row.get(0),
            date: row.get(1),
            inventory_number: row.get(2),
            name: row.get(3),
            model: row.get(4),
        })
    }
}

#[derive(Debug)]
pub struct TechEmployee {
    pub date: NaiveDate,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub job_title: String,
    pub inventory_number: i32,
    pub name: String,
    pub model: String,
}

impl TechEmployee {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            date: row.get(0),
            first_name: row.get(1),
            middle_name: row.get(2),
            last_name: row.get(3),
            job_title: row.get(4),
            inventory_number: row.get(5),
            name: row.get(6),
            model: row.get(7)
        })
    }
}

impl Tech {
    fn from_row(row: Row) -> Result<Tech, Error> {
        Ok(Tech {
            id: row.get(0),
            inventory_number: row.get(1),
            name: row.get(2),
            model: row.get(3),
            acquisition_date: row.get(4),
            price: row.get(5),
        })
    }

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(-1)
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn new(
        inventory_number: i32,
        name: &str,
        model: &str,
        acquisition_date: &str,
        price: i32,
    ) -> Tech {
        Tech {
            id: None,
            inventory_number,
            name: name.into(),
            model: model.into(),
            acquisition_date: acquisition_date.parse().expect("Invalid date"),
            price,
        }
    }

    pub fn new_with_date(
        inventory_number: i32,
        name: &str,
        model: &str,
        acquisition_date: NaiveDate,
        price: i32,
    ) -> Tech {
        Tech {
            id: None,
            inventory_number,
            name: name.into(),
            model: model.into(),
            acquisition_date,
            price,
        }
    }
}

pub struct TechDaoImpl {
    pool: Rc<PostgrePool>,
}

impl TechDaoImpl {
    pub fn new(pool: Rc<PostgrePool>) -> Self {
        Self { pool }
    }

    pub fn insert(&self, tech: &mut Tech) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();

        let id: i32 = conn
            .query_one(
                Self::INSERT,
                &[
                    &tech.inventory_number,
                    &tech.name,
                    &tech.model,
                    &tech.acquisition_date,
                    &tech.price,
                ],
            )
            .map(|row| row.get(0))?;

        tech.id = Some(id);

        Ok(())
    }

    pub fn update(&self, tech: &Tech) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(
            Self::UPDATE,
            &[
                &tech.inventory_number,
                &tech.name,
                &tech.model,
                &tech.acquisition_date,
                &tech.price,
                &tech.id,
            ],
        )?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Tech>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_tech = vec![];
        for row in conn.query(Self::FIND_ALL, &[])? {
            all_tech.push(Tech::from_row(row)?);
        }
        Ok(all_tech)
    }

    pub fn get_id(&self, id: i32) -> Result<Tech, Error> {
        let mut conn = self.pool.get().unwrap();
        let row = conn.query_one(Self::FIND_ID, &[&id])?;
        Ok(Tech::from_row(row)?)
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.pool.get().unwrap();
        let rows_affected = conn.execute(Self::DELETE, &[&id])?;
        Ok(())
    }

    pub fn get_by_unit_id(&self, unit_id: i32) -> Result<Vec<TechUnit>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_unit = vec![];
        for row in conn.query(Self::FIND_BY_UNIT_ID, &[&unit_id])? {
            all_unit.push(TechUnit::from_row(row)?);
        }
        Ok(all_unit)
    }

    pub fn get_by_employee_id(&self, employee_id: i32) -> Result<Vec<TechEmployee>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_unit = vec![];
        for row in conn.query(Self::FIND_BY_EMPLOYEE_ID, &[&employee_id])? {
            all_unit.push(TechEmployee::from_row(row)?);
        }
        Ok(all_unit)
    }

    const INSERT: &'static str = "INSERT INTO tech (inventory_number, name, model, acquisition_date, price) VALUES ($1, $2, $3, $4, $5) RETURNING id";
    const UPDATE: &'static str = "UPDATE tech SET inventory_number = $1, name = $2, model = $3, acquisition_date = $4, price = $5 WHERE id = $6";
    const FIND_ALL: &'static str =
        "SELECT id, inventory_number, name, model, acquisition_date, price FROM tech";
    const FIND_ID: &'static str =
        "SELECT id, inventory_number, name, model, acquisition_date, price FROM tech WHERE id = $1";
    const DELETE: &'static str = "DELETE FROM tech WHERE id = $1";

    const FIND_BY_UNIT_ID: &'static str = r#"SELECT 
        u.full_name AS unit_name,       
        tt.transfer_date AS transfer_date,
        t.inventory_number,          
        t.name,                           
        t.model
    FROM 
        transfer tt
    JOIN 
        employee e ON tt.employee_id = e.id  -- Correct join to the employee table
    JOIN 
        unit u ON e.unit_id = u.id           -- Correct join to the unit table    
    JOIN 
        tech t ON tt.tech_id = t.id  
    WHERE 
        u.id = $1    
        AND tt.transfer_date <= CURRENT_DATE 
        AND tt.id = (
    	    SELECT id
    	    FROM transfer
    	    WHERE tech_id = tt.tech_id
    	    AND transfer_date <= CURRENT_DATE
    	    ORDER BY transfer_date DESC
    	    LIMIT 1
	    )
    ORDER BY 
        tt.transfer_date DESC;  "#;

    const FIND_BY_EMPLOYEE_ID: &'static str = r#"SELECT 
        tt.transfer_date,                                                           
        e.first_name,
	    e.middle_name,
	    e.last_name AS employee_name,  
        e.job_title,                                                                
        t.inventory_number,      
        t.name,                                                       
        t.model                                                          
    FROM 
        transfer tt
    JOIN 
        employee e ON tt.employee_id = e.id  
    JOIN 
        tech t ON tt.tech_id = t.id      
    WHERE 
        e.id = $1                    
        AND tt.transfer_date <= CURRENT_DATE
        AND tt.id = (
    	    SELECT id
    	    FROM transfer
    	    WHERE tech_id = tt.tech_id
    	    AND transfer_date <= CURRENT_DATE
    	    ORDER BY transfer_date DESC
    	    LIMIT 1
	    ) 
    ORDER BY 
        tt.transfer_date DESC;"#;
}
