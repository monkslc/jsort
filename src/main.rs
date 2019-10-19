use serde_json::{Map, Value};
use std::io::{self, Read};

fn main() {
    let mut data = String::new();

    io::stdin()
        .read_to_string(&mut data)
        .expect("Couldn't read from stdin");

    let json: Map<String, Value> =
        serde_json::from_str(&data[..]).expect("Couldn't deserialize here");

    sort_and_print_json(&json, 0);

    // Print new line at the end
    println!("");
}

fn sort_and_print_json(json: &Map<String, Value>, indent_level: u32) {
    println!("{{");
    let mut keys = Vec::new();
    for key in json.keys() {
        keys.push(key);
    }
    keys.sort();
    let mut counter = 1;
    let len = keys.len();
    for key in keys {
        if let Value::Object(obj) = &json[key] {
            for _ in 0..indent_level + 1 {
                print!("\t");
            }
            print!("{}: ", key);
            sort_and_print_json(obj, indent_level + 1);
            if counter != len {
                println!(",");
            };
        } else {
            for _ in 0..indent_level + 1 {
                print!("\t");
            }
            if counter == len {
                println!("{}: {}", key, json[key]);
            } else {
                println!("{}: {},", key, json[key]);
            }
        }
        counter += 1
    }

    for _ in 0..indent_level {
        print!("\t");
    }

    print!("}}");
}
