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
