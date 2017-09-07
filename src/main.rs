extern crate play_combine;

use std::env;
use play_combine::parser;

fn main() {
    if let Some(s) = env::args().nth(1) {
        println!("{:?}", parser::parse(s.as_str()));
    }
    else {
        println!("No args");
    }
}
