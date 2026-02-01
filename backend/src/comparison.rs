use data_comparer_shared::{ComparisonResult, Dataset, MatchedRecord, Record};
use std::collections::HashMap;

pub fn compare_datasets(dataset1: Dataset, dataset2: Dataset) -> ComparisonResult {
    let mut result = ComparisonResult::new();

    let map1 = aggregate_by_id(dataset1.records);

    let map2 = aggregate_by_id(dataset2.records);

    let mut map1_clone = map1.clone();
    for (id, (name2, amount2)) in map2.iter() {
        if let Some((name1, amount1)) = map1_clone.remove(id) {
            result.matched.push(MatchedRecord {
                id: id.clone(),
                first_name: name1,
                first_amount: amount1,
                second_name: name2.clone(),
                second_amount: *amount2,
                amount_difference: amount2 - amount1,
            });
        } else {
            result
                .unmatched_from_second
                .push(Record::new(id.clone(), name2.clone(), *amount2));
        }
    }

    for (id, (name, amount)) in map1_clone {
        result
            .unmatched_from_first
            .push(Record::new(id, name, amount));
    }

    result
}

fn aggregate_by_id(records: Vec<Record>) -> HashMap<String, (String, f64)> {
    let mut map: HashMap<String, (String, f64)> = HashMap::new();

    for record in records {
        map.entry(record.id)
            .and_modify(|(_, amount)| *amount += record.amount)
            .or_insert((record.name, record.amount));
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregation() {
        let ds1 = Dataset::new(
            "Sales".to_string(),
            vec![Record::new("123".to_string(), "A".to_string(), 200.0)],
        );

        let ds2 = Dataset::new(
            "Payments".to_string(),
            vec![
                Record::new("123".to_string(), "A".to_string(), 20.0),
                Record::new("123".to_string(), "A".to_string(), 40.0),
            ],
        );

        let result = compare_datasets(ds1, ds2);

        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.matched[0].first_amount, 200.0);
        assert_eq!(result.matched[0].second_amount, 60.0);
        assert_eq!(result.matched[0].amount_difference, -140.0);
    }
}
