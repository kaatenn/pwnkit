use crate::database::Database;
use crate::error::PkError;
use std::path::PathBuf;

pub struct Template {
    pub name: String,
}

impl Template {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(".pwnkit")
            .join(self.name.clone() + &String::from(".py"))
    }
    
    pub fn list_templates() -> Result<(), PkError> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT name FROM templates ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

        println!("Template list:");
        for row in rows {
            println!("- {}", row?);
        }
        Ok(())
    }
}