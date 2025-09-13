use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Default)]
pub struct Competition {
    pub name: String,
    pub date: String,
}

impl Competition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            date: Local::now().to_rfc3339(),
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
