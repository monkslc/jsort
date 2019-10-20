use serde_json::{Map, Value};
use std::io::{self, Read};

fn main() {
    let mut data = String::new();

    io::stdin()
        .read_to_string(&mut data)
        .expect("Couldn't read from stdin");

    let json: Map<String, Value> =
        serde_json::from_str(&data[..]).expect("Couldn't deserialize here");

    let pretty_json = sort_json(&json, data.len());
    println!("{}", pretty_json);
}

fn sort_json(json: &Map<String, Value>, len: usize) -> String {
    let mut pretty_sorted_json = String::with_capacity(len);
    sort_json_recursive(json, 0, &mut pretty_sorted_json);

    pretty_sorted_json
}

fn sort_json_recursive(
    json: &Map<String, Value>,
    indent_level: usize,
    pretty_sorted_json: &mut String,
) {
    pretty_sorted_json.push_str("{\n");
    let mut keys = Vec::new();
    for key in json.keys() {
        keys.push(key);
    }
    keys.sort();

    let mut counter = 1;
    let len = keys.len();
    for key in keys {
        if let Value::Object(obj) = &json[key] {
            pretty_sorted_json.push_str(&"\t".repeat(indent_level + 1)[..]);
            pretty_sorted_json.push_str(&format!("{}: ", key)[..]);
            sort_json_recursive(obj, indent_level + 1, pretty_sorted_json);
            if counter != len {
                pretty_sorted_json.push_str(",\n");
            };
        } else {
            pretty_sorted_json.push_str(&"\t".repeat(indent_level + 1)[..]);
            if counter == len {
                pretty_sorted_json.push_str(&format!("{}: {}\n", key, json[key])[..]);
            } else {
                pretty_sorted_json.push_str(&format!("{}: {},\n", key, json[key])[..]);
            }
        }
        counter += 1
    }

    pretty_sorted_json.push_str(&"\t".repeat(indent_level)[..]);
    pretty_sorted_json.push_str("}");
}
