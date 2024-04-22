

/// Sqlite3 supported data types
pub enum SQLType {
    NULL,
    REAL(bool), // Specify if type must be unique or not
    TEXT(bool), // Specify if type must be unique or not
}
