use rusqlite::{Connection};
use serde::{Deserialize, Serialize};
use std::any::type_name;

 
pub struct Table<T> where T: for<'a> Deserialize<'a> + Serialize {
    name: String,
    data: T, 
}

impl<T> Table<T> where T: for<'a> Deserialize<'a> + Serialize {
    pub fn open(conn: &Connection, table_name: &String) -> Self {
        Self {
            name: table_name,
            data: todo!(),
        }
    }
}