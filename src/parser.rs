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
    Chunk(Vec<Box<Rule>>, Option<Box<Rule>>), // vec<stat>, laststat
    Block(Box<Rule>),
    Stat(
        StatKind,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
        Option<Box<Rule>>,
    ),
    LastStat(Box<Rule>),
    FuncName(Box<Rule>),
    Var(Box<Rule>),
    Exp(Box<Rule>),
    Prefixexp(Box<Rule>),               // (fc|var|exp)
    FunctionCall(Box<Rule>, Box<Rule>), // symbol, args
    Args(Box<Rule>),
    FuncBody(Option<Box<Rule>>, Box<Rule>), // params, block
    ParList1(Box<Rule>),                    // symbol(s)
    Nop,
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
        .skip(spaces())
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

pub fn args<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let nop = Box::new(Rule::Nop);
    between(token('('), token(')'), exp().or(value(nop))).map(|exp| Box::new(Rule::Args(exp)))
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

pub fn funcname<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    symbol().map(|name| Box::new(Rule::FuncName(name)))
}

pub fn funcbody<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        between(token('('), token(')'), parlist1()).skip(spaces()),
        block(),
    )
        .map(|(params, block)| Box::new(Rule::FuncBody(params, block)))
}

pub fn parlist1<Input>() -> impl Parser<Input, Output = Option<Box<Rule>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    symbol()
        .map(|name| Some(Box::new(Rule::ParList1(name))))
        .or(value(None))
}

pub fn stat<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        token(';').map(|_| Box::new(Rule::Stat(StatKind::Sep, None, None, None, None, None))),
        attempt(
            reserved("break")
                .map(|_| Box::new(Rule::Stat(StatKind::Break, None, None, None, None, None))),
        ),
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
        attempt(
            (
                reserved("function"),
                funcname(),
                funcbody(),
                reserved("end"),
            )
                .map(|(_, name, body, _)| {
                    Box::new(Rule::Stat(
                        StatKind::DeclareFunction,
                        name.into(),
                        body.into(),
                        None,
                        None,
                        None,
                    ))
                }),
        ),
    ))
}

pub fn laststat<Input>() -> impl Parser<Input, Output = Option<Box<Rule>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(
        (
            reserved("return"),
            exp()
                .map(|v| Some(Box::new(Rule::LastStat(v))))
                .or(value(None)),
        )
            .map(|(_, v)| v),
    )
}

pub fn chunk<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many(stat().skip(spaces())), laststat().or(value(None)))
        .map(|(ss, last): (Vec<Box<Rule>>, Option<Box<Rule>>)| Box::new(Rule::Chunk(ss, last)))
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
