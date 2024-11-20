use super::{Error, PostgrePool, Row};
use std::rc::Rc;

#[derive(Debug)]
pub struct Room {
    id: Option<i32>,
    pub room_number: i32,
    pub squares: i32,
}

impl Room {
    fn from_row(row: Row) -> Result<Room, Error> {
        Ok(Room {
            id: row.get(0),
            room_number: row.get(1),
            squares: row.get(2),
        })
    }

    pub fn to_str(&self) -> String {
        format!("number: {}, squares: {}", self.room_number, self.squares)
    }

    pub fn new(room_number: i32, squares: i32) -> Self {
        Self {
            id: None,
            room_number,
            squares,
        }
    }
    pub fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

pub struct RoomDAOImpl {
    pool: Rc<PostgrePool>,
}

impl RoomDAOImpl {
    pub fn new(pool: Rc<PostgrePool>) -> Self {
        Self { pool }
    }

    pub fn get_all(&self) -> Result<Vec<Room>, Error> {
        let mut conn = self.pool.get().unwrap();
        let mut all_rooms = vec![];
        for row in conn.query(Self::FIND_ALL, &[])? {
            all_rooms.push(Room::from_row(row)?);
        }
        Ok(all_rooms)
    }

    const FIND_ALL: &'static str = "SELECT id, room_number, squares FROM room;";
}
