use data_comparer_shared::{ComparisonResult, Dataset, MatchedRecord};
use std::collections::HashMap;

pub fn compare_datasets(dataset1: Dataset, dataset2: Dataset) -> ComparisonResult {
    let mut result = ComparisonResult::new();

    let mut map1: HashMap<String, (String, f64)> = HashMap::new();
    for record in dataset1.records {
        map1.insert(record.id.clone(), (record.name, record.amount));
    }

    for record2 in dataset2.records {
        if let Some((name1, amount1)) = map1.remove(&record2.id) {
            result.matched.push(MatchedRecord {
                id: record2.id,
                first_name: name1,
                first_amount: amount1,
                second_name: record2.name,
                second_amount: record2.amount,
                amount_difference: record2.amount - amount1,
            });
        } else {
            result.unmatched_from_second.push(record2);
        }
    }

    for (id, (name, amount)) in map1 {
        result
            .unmatched_from_first
            .push(data_comparer_shared::Record::new(id, name, amount));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use data_comparer_shared::Record;

    #[test]
    fn test_comparison() {
        let ds1 = Dataset::new(
            "Sales".to_string(),
            vec![
                Record::new("123".to_string(), "A".to_string(), 1000.0),
                Record::new("456".to_string(), "B".to_string(), 2000.0),
                Record::new("789".to_string(), "C".to_string(), 3000.0),
            ],
        );

        let ds2 = Dataset::new(
            "Payments".to_string(),
            vec![
                Record::new("123".to_string(), "A".to_string(), 1500.0),
                Record::new("456".to_string(), "B".to_string(), 2000.0),
                Record::new("999".to_string(), "D".to_string(), 4000.0),
            ],
        );

        let result = compare_datasets(ds1, ds2);

        assert_eq!(result.matched.len(), 2);
        assert_eq!(result.unmatched_from_first.len(), 1);
        assert_eq!(result.unmatched_from_second.len(), 1);
        assert_eq!(result.matched[0].amount_difference, 500.0);
    }
}
