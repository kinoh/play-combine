extern crate play_combine;

use std::env;
use play_combine::parser::TheParser;

fn main() {
    let parser = TheParser::new();

    if let Some(s) = env::args().nth(1) {
        println!("{:?}", parser.parse(s.as_str()));
    }
    else {
        println!("No args");
    }
}
