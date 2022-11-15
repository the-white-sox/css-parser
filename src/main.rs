use std::env;
use std::fs;

mod parser;
use crate::parser::parse;

fn main() {
    for file_name in env::args().skip(1) {
        let Ok(string) = fs::read_to_string(&file_name) else {
            eprintln!("Can not read file {}", file_name);
            continue;
        };

        println!("Reading {}", file_name);

        match parse(&string) {
            Ok(()) => (),
            Err(error) => eprintln!("{}", error),
        }
    }
}
