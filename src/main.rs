use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut data = String::new();

    io::stdin()
        .read_to_string(&mut data)
        .expect("Couldn't read from stdin");

    let json: HashMap<String, Value> =
        serde_json::from_str(&data[..]).expect("Couldn't deserialize here");

    let mut keys = Vec::new();
    for key in json.keys() {
        keys.push(key)
    }

    keys.sort();

    println!("{{");
    for key in keys {
        // In here we add a check if the json value is a hash map
        // If so, we need to loop through those keys and do the same process
        println!("\t{}: {}", key, json[key]);
    }
    println!("}}");
}
