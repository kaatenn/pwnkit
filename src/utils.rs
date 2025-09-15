use rusqlite::Connection;
use crate::database::Database;
use crate::error::DatabaseError;

pub fn connect() -> Result<Connection, DatabaseError> {
    Ok(Database::init_db().map_err(|e| DatabaseError::ConnectionError(e.to_string()))?)
}