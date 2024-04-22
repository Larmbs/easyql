use rusqlite::Connection;

mod table;
mod types;

use table::{Table};


struct SQLDataBase { 
    connection: Connection, 
    table: Vec<Table>,
}



#[cfg(test)]
mod tests;
