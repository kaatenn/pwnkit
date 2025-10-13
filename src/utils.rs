use crate::database::Database;
use crate::error::PkError;
use rusqlite::Connection;

pub fn connect() -> Result<Connection, PkError> {
    Ok(Database::init_db()?)
}