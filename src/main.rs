use serde_json::Value;
use std::collections::HashMap;

fn main() {
	
	let data = r#"{ "name": "Jon", "age": 37 }"#;	
	let v: HashMap<String, Value> = serde_json::from_str(data).expect("Couldn't deserialize here");
		
    println!("Name: {}, Age: {}", v["name"], v["age"]);
}
