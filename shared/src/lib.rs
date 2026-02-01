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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub matched: Vec<MatchedRecord>,
    pub unmatched_from_first: Vec<Record>,
    pub unmatched_from_second: Vec<Record>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedRecord {
    pub id: String,

    pub first_name: String,
    pub first_amount: f64,

    pub second_name: String,
    pub second_amount: f64,

    pub amount_difference: f64,
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
