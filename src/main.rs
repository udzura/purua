extern crate combine;

use std::env;

use combine::parser::char::{char, digit, space, spaces};
use combine::stream::position;

use combine::{chainl1, many1, skip_many, EasyParser, ParseError, Parser, Stream};

pub mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i32),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn number<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(digit())
        .skip(spaces())
        .map(|d: String| Box::new(Expr::Number(d.parse::<i32>().unwrap())))
}

fn muldiv<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = char('*').or(char('/')).skip(spaces()).map(|tok| {
        move |d1, d2| {
            if tok == '*' {
                Box::new(Expr::Mul(d1, d2))
            } else {
                Box::new(Expr::Div(d1, d2))
            }
        }
    });

    chainl1(number(), token)
}

fn binop<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = char('+').or(char('-')).skip(spaces()).map(|tok| {
        move |d1, d2| {
            if tok == '+' {
                Box::new(Expr::Plus(d1, d2))
            } else {
                Box::new(Expr::Minus(d1, d2))
            }
        }
    });

    chainl1(muldiv(), token)
}

fn myparser<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (spaces(), binop()).map(|(_, res)| res)
}

fn main() {
    let text = env::args().nth(1).unwrap_or("xxx = 1".to_string());
    match do_main(&text) {
        Ok(_) => println!("OK"),
        Err(err) => println!("{}", err),
    };
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    let mut parser = parser::stat();

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    println!("parsed: {:?}", res);

    Ok(())
}
