use std::fs::File;

mod parsers;
use parsers::csv_parser;

fn main() {
    println!("Testing CSV Parser...\n");

    let file = File::open("test_data.csv").expect("Failed to open test file");
    let dataset = csv_parser::parse_csv(file, "Sales".to_string()).expect("Failed to parse CSV");

    println!("Dataset name: {}", dataset.name);
    println!("Number of records: {}\n", dataset.records.len());

    for record in &dataset.records {
        println!(
            "ID: {}, Name: {}, Amount: {:.2}",
            record.id, record.name, record.amount
        );
    }

    println!("\n✅ CSV parser works!");
}
