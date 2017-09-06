extern crate play_combine;

use play_combine::parser::TheParser;

fn main() {
    let parser = TheParser::new();

    println!("{:?}", parser.parse("abc"));
    println!("{:?}", parser.parse("def"));
}
