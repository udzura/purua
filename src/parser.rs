extern crate combine;

use combine::parser::char::*;

use combine::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Nil,
    Numeral(i32),
    LiteralString(String),
    Symbol(String),
    Block(Vec<Box<Rule>>),
    Stat(
        StatKind,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
    ),
    Var(Box<Rule>),
    Exp(Box<Rule>),
    FunctionCall(Box<Rule>, Box<Rule>),
    Args(Box<Rule>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatKind {
    Sep,
    VarAssign,
    FunctionCall,
}

pub fn numeral<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(digit())
        .skip(spaces())
        .map(|d: String| Box::new(Rule::Numeral(d.parse().unwrap())))
}

pub fn literal_string<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(token('"'), token('"'), many(satisfy(|c| c != '"')))
        .map(|s: String| Box::new(Rule::LiteralString(s)))
}

pub fn symbol<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (letter(), many(alpha_num()))
        .skip(spaces())
        .map(|(c, v): (char, String)| Box::new(Rule::Symbol(format!("{}{}", c, v))))
}

pub fn var<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // choice((
    //     symbol(),
    //     (prefixexp(), char('['), exp(), char(']')),
    //     (prefixexp(), char('.'), symbol()),
    // ))
    symbol().map(|sym| Box::new(Rule::Var(sym)))
}

// pub fn prefixexp<Input>() -> impl Parser<Input, Output = Box<Rule>>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     unimplemented!()
// }

pub fn args<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let empty = Box::new(Rule::Nil);
    between(token('('), token(')'), exp().or(value(empty))).map(|exp| Box::new(Rule::Args(exp)))
}

pub fn functioncall<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (symbol(), args()).map(|(name, args)| Box::new(Rule::FunctionCall(name, args)))
}

pub fn exp<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((numeral(), literal_string())).map(|e| Box::new(Rule::Exp(e)))
}

pub fn stat<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        attempt((var(), token('=').skip(spaces()), exp())).map(|(v, _, e)| {
            Box::new(Rule::Stat(
                StatKind::VarAssign,
                v.into(),
                e.into(),
                None,
                None,
                None,
            ))
        }),
        attempt(functioncall()).map(|fc| {
            Box::new(Rule::Stat(
                StatKind::FunctionCall,
                fc.into(),
                None,
                None,
                None,
                None,
            ))
        }),
    ))
}

pub fn block<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(stat().skip(spaces())).map(|ss: Vec<Box<Rule>>| Box::new(Rule::Block(ss)))
}
