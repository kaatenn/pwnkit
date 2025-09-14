use chrono::Local;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use uuid::Uuid;


pub struct Competition {
    pub id: Option<String>,
    pub name: String,
    pub date: String,
}

impl Competition {
    pub fn new(name: String) -> Self {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            name,
            date: Local::now().to_rfc3339(),
        }
    }

    pub fn from_row(id: String, name: String, date: String) -> Self {
        Self {
            id: Some(id),
            name,
            date,
        }
    }
}

impl Display for Competition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.date)
    }
}

impl PartialEq for Competition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
