use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
// use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    #[serde(flatten)]
    fields: serde_json::Map<String, Value>,
}

fn main() -> Result<()> {
    let json_str = r#"
        {
            "age": 30,
            "name": "John",
            "city": "New York"
        }
    "#;

    let record: Record = serde_json::from_str(json_str).unwrap();
    let key = match record.fields.keys().next() {
        Some(key) => key,
        None => "age",
    };
    println!(
        "Deserialized: {} => {}",
        // record,
        key,
        record.fields.values().next().unwrap()
    );

    println!("Age: {}", record.fields.get("age").unwrap());

    let serialized = serde_json::to_string(&record).expect("Failed to serialize to struct");

    println!("Serialized {:?}", serialized);

    Ok(())
}
