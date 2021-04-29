extern crate combine;

use std::env;

use combine::parser::char::{char, digit, space, spaces};
use combine::stream::position;

use combine::{many1, skip_many, EasyParser, ParseError, Parser, Stream};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i32),
    Plus(Box<Expr>, Box<Expr>),
}

fn number<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(digit()).map(|d: String| Box::new(Expr::Number(d.parse::<i32>().unwrap())))
}

fn binop<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        number(),
        skip_many(space()),
        char('+').or(char('-')),
        skip_many(space()),
        number(),
    )
        .map(|(d1, _, tok, _, d2)| {
            if tok == '+' {
                Box::new(Expr::Plus(d1, d2))
            } else {
                unreachable!("TODO")
            }
        })
}

fn main() {
    let text = env::args().nth(1).expect("Usage command EXPR");
    match do_main(&text) {
        Ok(_) => println!("OK"),
        Err(err) => println!("{}", err),
    };
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    let mut parser = (spaces(), binop(), spaces()).map(|(_, res, _)| res.clone());

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    println!("parsed: {:?}", res);
    Ok(())
}
