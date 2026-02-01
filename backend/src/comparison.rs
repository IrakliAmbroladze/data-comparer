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
