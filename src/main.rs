use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

fn main() -> Result<()> {
    let json_str = r#"
        {
            "name": "John",
            "age": 30,
            "city": "New York"
        }
    "#;

    let record: Record = serde_json::from_str(json_str).unwrap();
    println!(
        "Deserialized: {:?}, {} => {}",
        record,
        record.fields.keys().next().unwrap(),
        record.fields.values().next().unwrap()
    );

    println!("Age: {}", record.fields.get("age").unwrap());

    let serialized = serde_json::to_string(&record).expect("Failed to serialize to struct");

    println!("Serialized {:?}", serialized);

    Ok(())
}
