use csv::ReaderBuilder;
use data_comparer_shared::{Dataset, Record};
use std::io::Read;

pub fn parse_csv<R: Read>(reader: R, dataset_name: String) -> anyhow::Result<Dataset> {
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut records = Vec::new();

    for result in csv_reader.records() {
        let record = result?;
        if record.len() != 3 {
            anyhow::bail!(
                "Expected 3 columns (id, name, amount), got {}",
                record.len()
            )
        }

        let id = record.get(0).unwrap().trim().to_string();
        let name = record.get(1).unwrap().trim().to_string();
        let amount = record.get(2).unwrap().trim().parse::<f64>()?;

        records.push(Record::new(id, name, amount));
    }
    Ok(Dataset::new(dataset_name, records))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let csv_data = "id,name,amount\n123,Company A,1500.00\n456,Company B,2300.50";
        let result = parse_csv(csv_data.as_bytes(), "Test".to_string()).unwrap();

        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].id, "123");
        assert_eq!(result.records[0].name, "Company A");
        assert_eq!(result.records[0].amount, 1500.00);
    }
}
