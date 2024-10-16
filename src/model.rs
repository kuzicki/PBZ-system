use rusqlite::{Connection, Result};
use crate::pool::SqlitePool;

#[derive(Debug)]
struct Employee {
    employee_id: u128,
    first_name: String,
    middle_name: String,
    last_name: String,
    is_supervisor: bool,
    is_accountable: bool,
    job_title: String,
    works_unit: u128,
    accountable_unit: u128,
}

trait EmployeeDAO {
    fn new(pool: SqlitePool) -> Self;
    fn add_supervisor(&self, employee: &Employee);
    fn add_accountable(&self, employee: &Employee);
    fn change_supervisor(&self, employee: &Employee);
    fn change_accountable(&self, employee: &Employee);
    fn remove_supervisor(&self, employee: &Employee);
    fn remove_accountable(&self, employee: &Employee);
    fn tech_list(&self, employee: &Employee) -> Vec<Tech>;
}

struct EmployeeDAOImpl {
    pool: SqlitePool
}

impl EmployeeDAO for EmployeeDAOImpl {
    const ADD: &'static str = "";
    const CHANGE: &'static str = "";
    const REMOVE: &'static str = "";
    const TECH_LIST: &'static str = "";

    fn new

    fn add_supervisor(&self, employee: &Employee) {
        
    }
}

#[derive(Debug)]
struct Unit {
    unit_number: u128,
    full_name: String,
    short_name: String,
    supervisor_id: u128,
}

trait UnitDAO {
    fn add(unit: &Unit);
    fn change(unit: &Unit);
    fn remove(unit_number: u128);
    fn room_list(unit_number: u128) -> Vec<Room>;
    fn tech_list(unit_number: u128) -> Vec<Tech>;
}

struct UnitDAOImpl {}

impl UnitDAO for UnitDAOImpl {
    const ADD: &str = "";
    const CHANGE: &str = "";
    const REMOVE: &str = "";
    const ROOM_LIST: &str = "";
    const TECH_LIST: &str = "";
}

#[derive(Debug)]
struct Transfer {
    transfer_date: String,
    inventory_number: u128,
    room_number: u128,
    unit_number: u128,
    employee_id: u128,
}

trait TransferDAO {
    fn add(transfer: &Transfer);
    fn change(transfer: &Transfer);
}

struct TransferDAOImpl {}

impl TransferDAO for TransferDAOImpl {
    const ADD: &str = "";
    const CHANGE: &str = "";
}

#[derive(Debug)]
struct Tech {
    inventory_number: u128,
    name: String,
    model: String,
    acquisition_date: String,
    price: u128,
}

trait TechDAO {
    fn add(tech: &Tech);
    fn change(tech: &Tech);
    fn remove(inventory_number: u128);

    const ADD: &'static str;
    const CHANGE: &'static str;
    const REMOVE: &'static str;
}

struct TechDaoImpl {}

impl TechDAO for TechDaoImpl {
    const ADD: &'static str = "";
    const CHANGE: &'static str = "";
    const REMOVE: &'static str = "";

    fn add(tech: &Tech) {

    }
}

#[derive(Debug)]
struct Room {
    room_number: u128,
    squares: u128,
}

trait RoomDAO {
    // fn add(room: &Room); // Not in the task
    const ADD: &'static str = "";
}

struct RoomDAOImpl {}

impl RoomDAO for RoomDAOImpl {
    const ADD: &'static str = "";
}
