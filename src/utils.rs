use crate::database::Database;
use crate::error::PkError;
use rusqlite::Connection;
use std::path::Path;
use std::process::Command;

pub fn connect() -> Result<Connection, PkError> {
    Ok(Database::init_db()?)
}

pub fn cd_into(path: &Path) -> Result<(), PkError> {
    if !path.exists() {
        return Err(PkError::ConfigError(format!(
            "Target directory does not exist: {}",
            path.display()
        )));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/K")
            .current_dir(path)
            .status()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash"));
        Command::new(shell).current_dir(path).status()?;
    }

    Ok(())
}
