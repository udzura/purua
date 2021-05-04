extern crate combine;

use combine::parser::char::*;
use combine::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Nil,
    Reserved(&'static str),
    Numeral(i32),
    LiteralString(String),
    Symbol(String),
    Chunk(Vec<Box<Rule>>),
    Block(Box<Rule>),
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
    Prefixexp(Box<Rule>),               // (fc|var|exp)
    FunctionCall(Box<Rule>, Box<Rule>), // symbol, args
    Args(Box<Rule>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatKind {
    Sep,
    VarAssign,
    FunctionCall,
    Label,
    Break,
    GoTo,
    Do,
    While,
    Repeat,
    IfThen,
    For,
    ForIn,
    DeclareFunction,
    LocalFunction,
    LocalVar,
}

pub fn reserved<Input>(word: &'static str) -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    string(word)
        .skip(spaces())
        .map(|s| Box::new(Rule::Reserved(s)))
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
        .then(|s: String| {
            let s = s.replace("\\n", "\n");
            value(s)
        })
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

parser! {
    pub fn exp[Input]() (Input) -> Box<Rule>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ] {
        choice((
            attempt(reserved("nil")),
            attempt(reserved("true")),
            attempt(reserved("false")),
            numeral(),
            literal_string(),
            prefixexp(),
        ))
            .map(|e| Box::new(Rule::Exp(e)))
    }
}

parser! {
    pub fn prefixexp[Input]() (Input) -> Box<Rule>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ] {
        choice((
            attempt(functioncall()),
            attempt(var()),
            between(token('('), token(')'), exp()),
        ))
            .map(|e| Box::new(Rule::Prefixexp(e)))
    }
}

pub fn stat<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        token(';').map(|_| Box::new(Rule::Stat(StatKind::Sep, None, None, None, None, None))),
        attempt(reserved("break"))
            .map(|_| Box::new(Rule::Stat(StatKind::Break, None, None, None, None, None))),
        attempt((reserved("do"), block(), reserved("end"))).map(|(_, blk, _)| {
            Box::new(Rule::Stat(StatKind::Do, blk.into(), None, None, None, None))
        }),
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

pub fn chunk<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(stat().skip(spaces())).map(|ss: Vec<Box<Rule>>| Box::new(Rule::Chunk(ss)))
}

parser! {
    pub fn block[Input]()(Input) -> Box<Rule>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ] {
        chunk().map(|blk| Box::new(Rule::Block(blk)))
    }
}
