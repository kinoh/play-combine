extern crate combine;

use self::combine::*;
use ::data::TheData;

pub struct TheParser {}

impl TheParser {
    pub fn new() -> Self {
        TheParser {}
    }

    fn parse_i32<I>(&self, input: I) -> ParseResult<i32, I>
        where I: Stream<Item=char> {

        many1::<String, _>(char::digit())
            .map(|s| s.parse::<i32>().unwrap())
            .parse_stream(input)
    }

    fn parse_num<I>(&self, input: I) -> ParseResult<i32, I>
        where I: Stream<Item=char> {

        char::string("num")
            .with(char::spaces())
            .with(parser(|x| self.parse_i32(x)))
            .parse_stream(input)
    }

    fn parse_vec<I>(&self, input: I) -> ParseResult<Vec<i32>, I>
        where I: Stream<Item=char> {

        char::string("vec")
            .with(char::spaces())
            .with(sep_by1(parser(|x| self.parse_i32(x)), token(',')))
            .parse_stream(input)
    }

    pub fn parse<I>(&self, input: I) -> ParseResult<TheData, I>
        where I: Stream<Item=char> {

        let mut p =
            sep_by(parser(|x| self.parse_num(x)), char::spaces())
            .skip(token(';'))
            .and(sep_by(parser(|x| self.parse_vec(x)), char::spaces()))
            .map(|t| {
                let (ns, vs) = t;
                TheData { num: ns, vec: vs }
            });

        p.parse_stream(input)
    }
}
