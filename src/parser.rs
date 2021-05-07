extern crate combine;

use combine::parser::char::*;
use combine::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Nil,
    Reserved(&'static str),
    Bool(bool),
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
    IfStat(Vec<Box<Rule>>, Vec<Box<Rule>>),
    FuncName(Box<Rule>),
    Var(Box<Rule>),
    Exp(Box<Rule>),
    Prefixexp(Box<Rule>),               // (fc|var|exp)
    FunctionCall(Box<Rule>, Box<Rule>), // symbol, args
    Args(Box<Rule>),
    FuncBody(Option<Box<Rule>>, Box<Rule>), // params, block
    ParList1(Box<Rule>),                    // symbol(s)
    BinOp(char, Box<Rule>, Box<Rule>),
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

pub fn nil<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    reserved("nil").map(|_| Box::new(Rule::Nil))
}

pub fn boolean<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    reserved("true")
        .map(|_| Box::new(Rule::Bool(true)))
        .or(reserved("false").map(|_| Box::new(Rule::Bool(false))))
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

pub fn binop1<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = choice((
        attempt(string("and").map(|_| '&')),
        attempt(string("or").map(|_| '|')),
    ))
    .skip(spaces())
    .map(|tok| move |d1, d2| Box::new(Rule::Exp(Box::new(Rule::BinOp(tok, d1, d2)))));
    chainl1(binop2(), token)
}

pub fn binop2<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = choice((
        attempt(string("<=").map(|_| 'g')),
        attempt(string(">=").map(|_| 'l')),
        char('<'),
        char('>'),
        char('-'),
        attempt(string("==").map(|_| 'e')),
        attempt(string("~=").map(|_| 'n')),
    ))
    .skip(spaces())
    .map(|tok| move |d1, d2| Box::new(Rule::Exp(Box::new(Rule::BinOp(tok, d1, d2)))));
    chainl1(binop3(), token)
}

pub fn binop3<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = char('+')
        .or(char('-'))
        .skip(spaces())
        .map(|tok| move |d1, d2| Box::new(Rule::Exp(Box::new(Rule::BinOp(tok, d1, d2)))));
    chainl1(binop4(), token)
}

pub fn binop4<Input>() -> impl Parser<Input, Output = Box<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let token = char('*')
        .or(char('/'))
        .skip(spaces())
        .map(|tok| move |d1, d2| Box::new(Rule::Exp(Box::new(Rule::BinOp(tok, d1, d2)))));
    chainl1(exp_(), token)
}

parser! {
    // For binop loop
    pub fn exp_[Input]() (Input) -> Box<Rule>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ] {
        choice((
            attempt(nil()),
            attempt(boolean()),
            numeral(),
            literal_string(),
            prefixexp(),
        ))
            .map(|e| Box::new(Rule::Exp(e)))
    }
}

parser! {
    pub fn exp[Input]() (Input) -> Box<Rule>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ] {
        binop1()
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
        )).skip(spaces())
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
            (
                reserved("if"),
                exp(),
                reserved("then"),
                block().skip(spaces()),
                many(
                    (
                        attempt(reserved("elseif")),
                        exp(),
                        reserved("then"),
                        block(),
                    )
                        .map(|(_, exp, _, blk)| (exp, blk)),
                )
                .or(value(vec![]))
                .skip(spaces()),
                (attempt(reserved("else")), block())
                    .or(value((Box::new(Rule::Nop), Box::new(Rule::Nop))))
                    .skip(spaces()),
                reserved("end"),
            )
                .map(
                    |(_, ifexp, _, thenblk, elifpairs, elsepair, _): (
                        _,
                        _,
                        _,
                        _,
                        Vec<(Box<Rule>, Box<Rule>)>,
                        (Box<Rule>, Box<Rule>),
                        _,
                    )| {
                        let mut vec0 = vec![ifexp];
                        let mut vec1 = vec![thenblk];
                        for (exp, blk) in elifpairs.into_iter() {
                            vec0.push(exp);
                            vec1.push(blk);
                        }
                        if let Rule::Block(_) = elsepair.1.as_ref() {
                            vec0.push(Box::new(Rule::Nop));
                            vec1.push(elsepair.1);
                        };
                        let ifst = Rule::IfStat(vec0, vec1);
                        Box::new(Rule::Stat(
                            StatKind::IfThen,
                            Box::new(ifst).into(),
                            None,
                            None,
                            None,
                            None,
                        ))
                    },
                ),
        ),
        attempt(
            reserved("break")
                .map(|_| Box::new(Rule::Stat(StatKind::Break, None, None, None, None, None))),
        ),
        attempt((reserved("do"), block(), reserved("end"))).map(|(_, blk, _)| {
            Box::new(Rule::Stat(StatKind::Do, blk.into(), None, None, None, None))
        }),
        attempt(
            (
                reserved("local"),
                symbol(),
                (token('=').skip(spaces()), exp())
                    .map(|(_, e)| e)
                    .or(value(Box::new(Rule::Exp(Box::new(Rule::Nil))))),
            )
                .map(|(_, name, val)| {
                    Box::new(Rule::Stat(
                        StatKind::LocalVar,
                        name.into(),
                        val.into(),
                        None,
                        None,
                        None,
                    ))
                }),
        ),
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
