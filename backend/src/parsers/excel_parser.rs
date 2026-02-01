use calamine::{open_workbook_auto_from_rs, Data, Reader, Sheets};
use data_comparer_shared::{Dataset, Record};
use std::io::{Read, Seek};

pub fn parse_excel<R: Read + Seek + Clone>(
    reader: R,
    dataset_name: String,
) -> anyhow::Result<Dataset> {
    let mut workbook: Sheets<R> = open_workbook_auto_from_rs(reader)?;

    let sheet_names = workbook.sheet_names().to_owned();
    let sheet_name = &sheet_names[0];

    let range = workbook.worksheet_range(sheet_name)?;

    let mut records = Vec::new();
    let rows: Vec<_> = range.rows().collect();

    for row in rows.iter().skip(1) {
        if row.len() < 3 {
            continue;
        }

        let id = cell_to_string(&row[0]);
        let name = cell_to_string(&row[1]);
        let amount = cell_to_f64(&row[2])?;

        if !id.is_empty() {
            records.push(Record::new(id, name, amount));
        }
    }

    Ok(Dataset::new(dataset_name, records))
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::String(s) => s.trim().to_string(),
        Data::Int(i) => i.to_string(),
        Data::Float(f) => f.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::Empty => String::new(),
        _ => String::new(),
    }
}

fn cell_to_f64(cell: &Data) -> anyhow::Result<f64> {
    match cell {
        Data::Float(f) => Ok(*f),
        Data::Int(i) => Ok(*i as f64),
        Data::String(s) => s
            .trim()
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Cannot parse '{}' as number", s)),
        _ => anyhow::bail!("Invalid amount value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_parse_excel() {
        let file = File::open("test_data.xlsx");
        if let Ok(f) = file {
            let result = parse_excel(f, "Test".to_string()).unwrap();
            assert!(result.records.len() > 0);
        }
    }
}
