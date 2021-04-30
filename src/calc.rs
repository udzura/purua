use combine::parser::char::{char, digit, spaces};
use combine::{chainl1, many1, ParseError, Parser, Stream};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
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

pub fn calc_parser<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (spaces(), binop()).map(|(_, res)| res)
}
