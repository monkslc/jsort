use std::fs;

use serde_json;
use std::io::{self, Read};
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // configure command line + get arguments
    let yaml = load_yaml!("config/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let json_content = match matches.value_of("File") {
        Some(filename) => read_file_or_panic(filename),
        None => read_from_stdin(),
    };

    let json_object: serde_json::Value = match serde_json::from_str(&json_content[..]) {
        Ok(json_object) => json_object,
        Err(error) => panic!("Not valid json {}", error),
    };

    // unwrapping because we know its valid json
    let formatted_json = serde_json::to_string_pretty(&json_object).unwrap();
    println!("{}", formatted_json);
}

fn read_file_or_panic(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file_content) => file_content,
        Err(error) => panic!("Error reading file: {}", error),
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(error) => panic!("Error reading from stdin: {}", error),
    }
}
