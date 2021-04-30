extern crate combine;

use combine::parser::char::*;
use combine::stream::position;

use combine::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Numeral(i32),
    Symbol(String),
    Var(Box<Expr>),
    Exp(Box<Expr>),
    StatVarAssign(Box<Expr>, Box<Expr>),
}

pub fn numeral<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(digit())
        .skip(spaces())
        .map(|d: String| Box::new(Expr::Numeral(d.parse().unwrap())))
}

pub fn symbol<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (letter(), many1(alpha_num()))
        .skip(spaces())
        .map(|(c, v): (char, String)| Box::new(Expr::Symbol(format!("{}{}", c, v))))
}

pub fn var<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // choice((
    //     symbol(),
    //     (prefixexp(), char('['), exp(), char(']')),
    //     (prefixexp(), char('.'), symbol()),
    // ))
    symbol().map(|sym| Box::new(Expr::Var(sym)))
}

// pub fn prefixexp<Input>() -> impl Parser<Input, Output = Box<Expr>>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     unimplemented!()
// }

pub fn exp<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    //choice((numeral(), ...))
    numeral().map(|sym| Box::new(Expr::Exp(sym)))
}

pub fn stat<Input>() -> impl Parser<Input, Output = Box<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (var(), token('=').skip(spaces()), exp()).map(|(v, _, e)| Box::new(Expr::StatVarAssign(v, e)))
}
