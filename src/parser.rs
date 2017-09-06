extern crate combine;

use self::combine::*;

pub struct TheParser {}

impl TheParser {
    pub fn new() -> Self {
        TheParser {}
    }

    pub fn parse<I>(&self, input: I) -> ParseResult<(), I>
        where I: Stream<Item=char> {

        let mut p = token('a').map(|_| ());

        p.parse_stream(input)
    }
}
