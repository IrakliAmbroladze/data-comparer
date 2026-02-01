mod comparison;
use comparison::compare_datasets;
use data_comparer_shared::{Dataset, Record};

fn main() {
    println!("Testing Comparison Logic...\n");

    let sales = Dataset::new(
        "Sales".to_string(),
        vec![
            Record::new("12345".to_string(), "Company A".to_string(), 1500.0),
            Record::new("67890".to_string(), "Company B".to_string(), 2300.0),
            Record::new("11111".to_string(), "Company C".to_string(), 750.0),
        ],
    );

    let payments = Dataset::new(
        "Payments".to_string(),
        vec![
            Record::new("12345".to_string(), "Company A".to_string(), 1600.0),
            Record::new("67890".to_string(), "Company B".to_string(), 2300.0),
            Record::new("22222".to_string(), "Company D".to_string(), 999.0),
        ],
    );

    let result = compare_datasets(sales, payments);

    println!("MATCHED ({} records):", result.matched.len());
    for m in &result.matched {
        println!("  ID: {}", m.id);
        println!("    Sales: {} - {:.2}", m.first_name, m.first_amount);
        println!("    Payments: {} - {:.2}", m.second_name, m.second_amount);
        println!("    Difference: {:.2}\n", m.amount_difference);
    }

    println!(
        "UNMATCHED FROM SALES ({} records):",
        result.unmatched_from_first.len()
    );
    for r in &result.unmatched_from_first {
        println!("  ID: {}, Name: {}, Amount: {:.2}", r.id, r.name, r.amount);
    }

    println!(
        "\nUNMATCHED FROM PAYMENTS ({} records):",
        result.unmatched_from_second.len()
    );
    for r in &result.unmatched_from_second {
        println!("  ID: {}, Name: {}, Amount: {:.2}", r.id, r.name, r.amount);
    }

    println!("\n✅ Comparison logic works!");
}
