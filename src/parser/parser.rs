use combine::{many1, optional, parser, token, Parser, Stream, StreamOnce, ParseError};

use crate::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use super::ast::*;
use super::stream::TokenStream;

pub fn parse(stream: TokenStream) -> Result<Block, String> {
    let mut parser = block();
    let result = parser.parse(stream);
    match &result {
        Ok((block, _)) => {
            dbg!(block);
            Ok(block.clone())
        },
        Err(err) => Err(format!("Parse error: {:?}", err)),
    }
}

pub fn parse_dummy(stream: TokenStream) -> Result<(), String> {
    let c: Token = Int.into();
    let tok = token(c);
    let eof: Token = Eof.into();
    let mut parser= (many1(tok).map(|v: Vec<Token>| v), token(eof));

    let result: Result<(_, TokenStream), super::stream::TokenStreamError> = parser.parse(stream);
    match result {
        Ok((tok, _)) => {
            dbg!(tok);
            Ok(())
        },
        Err(err) => Err(format!("Parse error: {:?}", err)),
    }    
}

parser! {
    fn block[Input]()(Input) -> Block
    where [
        Input: Stream<Token = Token>,
    ] {
        chunk().map(|chunk| {
            Block(chunk)
        })
    }
}

fn chunk<Input>() -> impl Parser<Input, Output = Chunk>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    many1(stat()).and(optional(last_stat()))
        .map(|(stat, last_stat)| {
            Chunk(stat, last_stat)
        })
}

fn stat<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let assign = stat_assign();
    // let function_call = function_call();
    let do_block = stat_do_block();
    // let while_block = while_block();
    // let repeat_block = repeat_block();
    // let if_block = if_block();
    // let for_block = for_block();
    // let for_in_block = for_in_block();
    // let function_decl = function_decl();
    // let local_function_decl = local_function_decl();
    // let local_var_decl = local_var_decl();

    assign.or(do_block)
        // .or(while_block)
        // .or(repeat_block)
        // .or(if_block)
        // .or(for_block)
        // .or(for_in_block)
        // .or(function_decl)
        // .or(local_function_decl)
        // .or(local_var_decl)
}

fn stat_assign<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let varlist = varlist();
    let exprlist = exprlist();
    (varlist, token(TokenType::Assign.into()), exprlist)
        .map(|(varlist, _, exprlist)| Stat::Assign(varlist, exprlist))
}

fn stat_do_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let block = block();
    (token(TokenType::Do.into()), block, token(TokenType::End.into()))
        .map(|(_, block, _)| Stat::Do(block))
}

fn last_stat<Input>() -> impl Parser<Input, Output = LastStat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let return_stat = token(TokenType::Return.into())
        .and(optional(exprlist()))
        .map(|(_, exprlist)| LastStat::Return(exprlist));
    let break_stat = token(TokenType::Break.into())
        .map(|_| LastStat::Break);
    return_stat.or(break_stat)
}

fn varlist<Input>() -> impl Parser<Input, Output = VarList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    many1(var()).map(|vars| VarList(vars))
}

fn var<Input>() -> impl Parser<Input, Output = Var>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name = token(TokenType::Name.into()).map(|name| Var::VarName(name));
    // let prefixexp = prefixexp();
    // let index = token(TokenType::LBracket.into())
    //     .with(expr())
    //     .skip(token(TokenType::RBracket.into()));
    // let dot_name = token(TokenType::Dot.into())
    //     .with(token(TokenType::Name.into()));
    // name.or(prefixexp.and(index).map(|(prefix, index)| Var::Index(prefix, index)))
    //     .or(prefixexp.and(dot_name).map(|(prefix, name)| Var::Dot(prefix, name)))
    name
}

fn exprlist<Input>() -> impl Parser<Input, Output = ExprList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    many1(expr()).map(|exprs| ExprList(exprs))
}

fn expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >, 
{
    let nil = token(TokenType::Nil.into()).map(|_| Expr::Nil);
    let false_expr = token(TokenType::False.into()).map(|_| Expr::False);
    let true_expr = token(TokenType::True.into()).map(|_| Expr::True);
    let number = token(TokenType::Int.into()).map(|num: Token| Expr::Number(num.try_into().unwrap()));
    let string = token(TokenType::StringLit.into()).map(|s: Token| Expr::String(s.try_into().unwrap()));
    let dots = token(TokenType::Dots.into()).map(|_| Expr::Dots);
    nil.or(false_expr)
        .or(true_expr)
        .or(number)
        .or(string)
        .or(dots)
}
