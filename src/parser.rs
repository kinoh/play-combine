extern crate combine;

use self::combine::*;
use ::data::TheData;

#[derive(Debug)]
enum Token {
    Num(i32),
    Vec(Vec<i32>),
}

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

pub fn parse<I>(input: I) -> ParseResult<TheData, I>
    where I: Stream<Item=char> {

    let mut p =
        sep_by::<Vec<Token>, _, _>(
            parser(parse_num).map(Token::Num)
                .or(parser(parse_vec).map(Token::Vec)),
            token(';'))
            .map(|ts| {
                let mut data = TheData { num: vec![], vec: vec![] };
                for t in ts {
                    match t {
                        Token::Num(n) => data.num.push(n),
                        Token::Vec(v) => data.vec.push(v),
                    }
                }
                data
            });

    p.parse_stream(input)
}
