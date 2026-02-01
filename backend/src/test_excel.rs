use std::fs::File;
use std::io::Read;

mod parsers;
use parsers::excel_parser;

fn main() {
    println!("Testing Excel Parser...\n");

    let mut file = File::open("test_data.xlsx").expect("Failed to open test file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let dataset = excel_parser::parse_excel(std::io::Cursor::new(buffer), "Sales".to_string())
        .expect("Failed to parse Excel");

    println!("Dataset name: {}", dataset.name);
    println!("Number of records: {}\n", dataset.records.len());

    for record in &dataset.records {
        println!(
            "ID: {}, Name: {}, Amount: {:.2}",
            record.id, record.name, record.amount
        );
    }

    println!("\n✅ Excel parser works!");
}
