extern crate combine;

use self::combine::*;
use ::data::TheData;

fn parse_i32<I>(input: I) -> ParseResult<i32, I>
    where I: Stream<Item=char> {

    many1::<String, _>(char::digit())
        .map(|s| s.parse::<i32>().unwrap())
        .parse_stream(input)
}

fn parse_num<I>(input: I) -> ParseResult<i32, I>
    where I: Stream<Item=char> {

    char::string("num")
        .with(char::spaces())
        .with(parser(parse_i32))
        .parse_stream(input)
}

fn parse_vec<I>(input: I) -> ParseResult<Vec<i32>, I>
    where I: Stream<Item=char> {

    char::string("vec")
        .with(char::spaces())
        .with(sep_by1(parser(parse_i32), token(',')))
        .parse_stream(input)
}

pub struct TheParser {}

impl TheParser {
    pub fn new() -> Self {
        TheParser {}
    }

    pub fn parse<I>(&self, input: I) -> ParseResult<TheData, I>
        where I: Stream<Item=char> {

        let mut p =
            sep_by(parser(parse_num), char::spaces())
            .skip(token(';'))
            .and(sep_by(parser(parse_vec), char::spaces()))
            .map(|t| {
                let (ns, vs) = t;
                TheData { num: ns, vec: vs }
            });

        p.parse_stream(input)
    }
}
