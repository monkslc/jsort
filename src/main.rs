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

    let content = match matches.value_of("file") {
        Some(filename) => read_file_or_panic(filename),
        None => read_from_stdin(),
    };

    let pretty_json = get_pretty_json_or_panic(content);
    println!("{}", pretty_json);
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

fn get_pretty_json_or_panic(content: String) -> String {
    let json_object: serde_json::Value = match serde_json::from_str(&content[..]) {
        Ok(json_object) => json_object,
        Err(error) => panic!("Not valid json {}", error),
    };

    // unwrapping because we know its valid json
    serde_json::to_string_pretty(&json_object).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_json_test() {
        test_json_files(
            String::from("simple.json"),
            String::from("simple_verify.json"),
        );
    }

    #[test]
    fn nested_json_test() {
        test_json_files(
            String::from("nested.json"),
            String::from("nested_verify.json"),
        );
    }

    #[test]
    fn array_json_test() {
        test_json_files(
            String::from("array.json"),
            String::from("array_verify.json"),
        );
    }

    fn test_json_files(file: String, verify_file: String) {
        let content = read_file_or_panic(&format!("sample_json/{}", file)[..]);
        let mut expected = read_file_or_panic(&format!("sample_json/{}", verify_file)[..]);
        expected.pop(); // remove extra new line character from file
        assert_eq!(expected, get_pretty_json_or_panic(content));
    }
}
