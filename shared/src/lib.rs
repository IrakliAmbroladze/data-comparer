use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub records: Vec<Record>,
}

impl Record {
    pub fn new(id: String, name: String, amount: f64) -> Self {
        Self { id, name, amount }
    }
}

impl Dataset {
    pub fn new(name: String, records: Vec<Record>) -> Self {
        Self { name, records }
    }

    pub fn empty(name: String) -> Self {
        Self {
            name,
            records: Vec::new(),
        }
    }
}
