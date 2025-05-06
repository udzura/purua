use combine::{
    attempt, chainl1, many, optional, parser, sep_by, sep_by1, token, ParseError, Parser, Stream, StreamOnce
};

use super::ast;
use super::ast::*;
use super::stream::TokenStream;
use crate::token_type::TokenType;
use crate::Token;

pub fn parse(stream: TokenStream) -> Result<Block, String> {
    let mut parser = (block(), token(TokenType::Eof.into()));
    let result = parser.parse(stream);

    match &result {
        Ok(((block, _), _)) => {
            dbg!(block);
            Ok(block.clone())
        }
        Err(err) => {
            Err(format!("Parse error: {:?}", err))
        }
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
    many(stat())
        .and(optional(laststat()))
        .map(|(stat, last_stat)| Chunk(stat, last_stat))
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
    let function_call = stat_function_call();
    let do_block = stat_do_block();
    let while_block = stat_while_block();
    let repeat_block = stat_repeat_block();
    let if_block = stat_if_block();
    let for_block = stat_for_block();
    let for_in_block = stat_for_in_block();
    let function_decl = stat_function_decl();
    let local_function_decl = stat_local_function_decl();
    let local_var_decl = stat_local_var_decl();

    attempt(assign)
        .or(attempt(
            function_call
        ))
        .or(do_block)
        .or(while_block)
        .or(repeat_block)
        .or(if_block)
        .or(for_block)
        .or(for_in_block)
        .or(function_decl)
        .or(local_function_decl)
        .or(local_var_decl)
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
    let exprlist = exprlist1();
    (varlist, token(TokenType::Assign.into()), exprlist)
        .map(|(varlist, _, exprlist)| Stat::Assign(varlist, exprlist))
}

fn stat_function_call<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let function_call = functioncall();
    function_call.map(|function_call| Stat::FunctionCall(function_call))
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
    (
        token(TokenType::Do.into()),
        block,
        token(TokenType::End.into()),
    )
        .map(|(_, block, _)| Stat::Do(block))
}

fn stat_while_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let expr = expr_binop_bottom();
    let block = block();
    (
        token(TokenType::While.into()),
        expr,
        token(TokenType::Do.into()),
        block,
        token(TokenType::End.into()),
    )
        .map(|(_, expr, _, block, _)| Stat::While(Box::new(expr), block))
}

fn stat_repeat_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let block = block();
    let expr = expr_binop_bottom();
    (
        token(TokenType::Repeat.into()),
        block,
        token(TokenType::Until.into()),
        expr,
    )
        .map(|(_, block, _, expr)| Stat::Repeat(Box::new(expr), block))
}

fn stat_if_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let expr_elseif = expr_binop_bottom();
    let expr = expr_binop_bottom();
    let block_elseif = block();
    let block_else = block();
    let block = block();
    let elseif_block = many(
        (
            token(TokenType::Elseif.into()),
            expr_elseif,
            token(TokenType::Then.into()),
            block_elseif,
        )
            .map(|(_, expr, _, block)| (Box::new(expr), block)),
    );
    let else_block = optional((token(TokenType::Else.into()), block_else).map(|(_, block)| block));
    (
        token(TokenType::If.into()),
        expr,
        token(TokenType::Then.into()),
        block,
        elseif_block,
        else_block,
        token(TokenType::End.into()),
    )
        .map(|(_, expr, _, block, elseif_block, else_block, _)| {
            Stat::If(Box::new(expr), block, elseif_block, else_block)
        })
}

fn stat_for_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name = token(TokenType::Name.into());
    let expr_init = expr_binop_bottom();
    let expr_cond = expr_binop_bottom();
    let expr_incr = expr_binop_bottom();
    let block = block();
    (
        token(TokenType::For.into()),
        name,
        token(TokenType::Assign.into()),
        expr_init,
        token(TokenType::Comma.into()),
        expr_cond,
        optional(token(TokenType::Comma.into()).with(expr_incr)),
        token(TokenType::Do.into()),
        block,
        token(TokenType::End.into()),
    )
        .map(|(_, name, _, expr1, _, expr2, expr3, _, block, _)| {
            Stat::For(
                name,
                Box::new(expr1),
                Box::new(expr2),
                expr3.map(Box::new),
                block,
            )
        })
}

fn stat_for_in_block<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name_list = namelist();
    let expr_list = exprlist();
    let block = block();
    (
        token(TokenType::For.into()),
        name_list,
        token(TokenType::In.into()),
        expr_list,
        token(TokenType::Do.into()),
        block,
        token(TokenType::End.into()),
    )
        .map(|(_, name_list, _, expr_list, _, block, _)| Stat::ForIn(name_list, expr_list, block))
}

fn stat_function_decl<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let func_name = funcname();
    let func_body = funcbody();
    (token(TokenType::Function.into()), func_name, func_body)
        .map(|(_, func_name, func_body)| Stat::Function(func_name, func_body))
}

fn stat_local_function_decl<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name = token(TokenType::Name.into());
    let func_body = funcbody();
    (
        token(TokenType::Local.into()),
        token(TokenType::Function.into()),
        name,
        func_body,
    )
        .map(|(_, _, name, func_body)| Stat::LocalFunction(name, func_body))
}

fn stat_local_var_decl<Input>() -> impl Parser<Input, Output = Stat>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name_list = namelist();
    let expr_list = optional(token(TokenType::Assign.into()).with(exprlist1()));
    (token(TokenType::Local.into()), name_list, expr_list)
        .map(|(_, name_list, expr_list)| Stat::LocalDeclVar(name_list, expr_list))
}

fn laststat<Input>() -> impl Parser<Input, Output = LastStat>
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
    let break_stat = token(TokenType::Break.into()).map(|_| LastStat::Break);
    return_stat.or(break_stat)
}

fn funcname<Input>() -> impl Parser<Input, Output = FuncName>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name = token(TokenType::Name.into());
    let dot = token(TokenType::Period.into());
    let colon_name = token(TokenType::Colon.into()).with(token(TokenType::Name.into()));
    sep_by1(name, dot)
        .and(optional(colon_name))
        .map(|(names, colon)| FuncName(names, colon))
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
    sep_by1(var(), token(TokenType::Comma.into())).map(|vars| VarList(vars))
}

parser! {
    fn var[Input]()(Input) -> Var
    where [
        Input: Stream<Token = Token>,
    ] {
        let name = token(TokenType::Name.into()).map(|name| Var::VarName(name));
        name
        // let prefixexp_idx = prefixexp();
        // let prefixexp_mem = prefixexp();
        // let index = token(TokenType::BracketL.into())
        //     .with(expr_binop_bottom())
        //     .skip(token(TokenType::BracketR.into()));
        // let dot_name = token(TokenType::Period.into())
        //     .with(token(TokenType::Name.into()));
        // name.or(prefixexp_idx.and(index).map(|(prefix, index)| Var::VarIdx(prefix, Box::new(index))))
        //     .or(prefixexp_mem.and(dot_name).map(|(prefix, name)| Var::VarMember(prefix, name)))
    }
}

fn namelist<Input>() -> impl Parser<Input, Output = NameList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    sep_by1(
        token(TokenType::Name.into()),
        token(TokenType::Comma.into()),
    )
    .map(|names| NameList(names))
}

fn exprlist1<Input>() -> impl Parser<Input, Output = ExprList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    sep_by1(expr_binop_bottom(), token(TokenType::Comma.into())).map(|exprs| ExprList(exprs))
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
    sep_by(expr_binop_bottom(), token(TokenType::Comma.into())).map(|exprs| ExprList(exprs))
}

parser! {
    fn expr[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = Token>,
    ] {
        attempt(expr_upper())
            .or(expr_lower())
    }
}

fn expr_upper<Input>() -> impl Parser<Input, Output = Expr>
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
    let number = token(TokenType::Int.into())
        .map(|num: Token| Expr::Number(num.try_into().unwrap()))
        .or(token(TokenType::Float.into()).map(|num: Token| Expr::Number(num.try_into().unwrap())));
    let string =
        token(TokenType::StringLit.into()).map(|s: Token| Expr::String(s.try_into().unwrap()));
    let dots = token(TokenType::Dots.into()).map(|_| Expr::Dots);
    nil.or(false_expr)
            .or(true_expr)
            .or(number)
            .or(string)
            .or(dots)
}


fn expr_lower<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let function = function().map(|fun| Expr::Function(fun));
    function
        .or(prefixexp().map(|prefix| Expr::PrefixExp(prefix)))
        .or(tableconstructor().map(|table| Expr::TableConstructor(table)))
        .or(unop().and(expr_binop_bottom()).map(|(unop, expr)| Expr::Unop(unop, Box::new(expr))))
}

parser! {
    fn prefixexp[Input]()(Input) -> PrefixExp
    where [
        Input: Stream<Token = Token>,
    ] {
        let var = var();
        let function_call = functioncall();
        let paren = token(TokenType::ParenL.into()).with(expr_binop_bottom()).skip(token(TokenType::ParenR.into()));
        attempt(function_call.map(|call| PrefixExp::PrefixCall(call)))
            .or(var.map(|var| PrefixExp::PrefixVar(Box::new(var))))
            .or(paren.map(|expr| PrefixExp::PrefixParen(Box::new(expr))))
    }
}

parser! {
    fn prefixexp_lowered[Input]()(Input) -> PrefixExp
    where [
        Input: Stream<Token = Token>,
    ] {
        let var = var();
        let paren = token(TokenType::ParenL.into()).with(expr_binop_bottom()).skip(token(TokenType::ParenR.into()));
        var.map(|var| PrefixExp::PrefixVar(Box::new(var)))
            .or(paren.map(|expr| PrefixExp::PrefixParen(Box::new(expr))))
    }
}

parser! {
    fn functioncall[Input]()(Input) -> FunctionCall
    where [
        Input: Stream<Token = Token>,
    ] {
        let prefixexp = prefixexp_lowered();
        let args = args();
        (
            prefixexp,
            optional(token(TokenType::Colon.into()).with(token(TokenType::Name.into()))),
            args,
        )
            .map(|(prefix, name, args)| FunctionCall(Box::new(prefix), name, args))
    }
}

fn args<Input>() -> impl Parser<Input, Output = Args>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let expr_list = exprlist();
    let table_constructor = tableconstructor().map(|table| Args::ArgsTable(table));
    let string =
        token(TokenType::StringLit.into()).map(|s: Token| Args::ArgsString(s.try_into().unwrap()));
    token(TokenType::ParenL.into())
        .with(expr_list)
        .skip(token(TokenType::ParenR.into()))
        .map(|expr_list| Args::ArgsList(expr_list))
        .or(table_constructor)
        .or(string)
}

parser! {
    fn function[Input]()(Input) -> ast::Function
    where [
        Input: Stream<Token = Token>,
    ] {
        let func_body = funcbody();
        token(TokenType::Function.into()).with(func_body).map(|func_body| ast::Function(func_body))
    }
}

fn funcbody<Input>() -> impl Parser<Input, Output = FuncBody>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let param_list = paramlist();
    let block = block();
    (
        token(TokenType::ParenL.into()),
        param_list,
        token(TokenType::ParenR.into()),
        block,
        token(TokenType::End.into()),
    )
        .map(|(_, param_list, _, block, _)| FuncBody(param_list, block))
}

fn paramlist<Input>() -> impl Parser<Input, Output = ParamList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let name_list = namelist();
    let dots = token(TokenType::Dots.into());
    let vararg = optional(dots).map(|_| true);
    (name_list, vararg).map(|(name_list, vararg)| ParamList(name_list, vararg))
}

fn tableconstructor<Input>() -> impl Parser<Input, Output = TableConstructor>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let field_list = fieldlist();
    (
        token(TokenType::BraceL.into()),
        field_list,
        token(TokenType::BraceR.into()),
    )
        .map(|(_, field_list, _)| TableConstructor(field_list))
}

fn fieldlist<Input>() -> impl Parser<Input, Output = FieldList>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let field = field();
    sep_by1(field, fieldsep())
        .skip(optional(fieldsep()))
        .map(|fields| FieldList(fields))
}

fn field<Input>() -> impl Parser<Input, Output = Field>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let index = token(TokenType::BracketL.into())
        .with(expr_binop_bottom())
        .skip(token(TokenType::BracketR.into()));
    let assign = || token(TokenType::Assign.into());
    let name = token(TokenType::Name.into());
    let field_assign = (index, assign(), expr_binop_bottom())
        .map(|(index, _, expr)| Field::AssignIdx(Box::new(index), Box::new(expr)));
    let field_name =
        (name, assign(), expr_binop_bottom()).map(|(name, _, expr)| Field::AssignName(name, Box::new(expr)));
    let field_expr = expr_binop_bottom().map(|expr| Field::UniExp(Box::new(expr)));
    field_assign.or(field_name).or(field_expr)
}

fn fieldsep<Input>() -> impl Parser<Input, Output = Fieldsep>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    token(TokenType::Comma.into())
        .or(token(TokenType::SemiColon.into()))
        .map(|_| Fieldsep)
}

// TODO: precedence!
// TODO: this is the "root" expr, rename it.
fn expr_binop_bottom<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let binop = attempt(binop()
        .map(|binop| |l, r| {
            Expr::Binop(Box::new(l), binop, Box::new(r))
        }));
    chainl1(expr(), binop)
}

// TODO: precedence!
fn binop<Input>() -> impl Parser<Input, Output = Binop>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let op = token(TokenType::Plus.into())
        .or(token(TokenType::Minus.into()))
        .or(token(TokenType::Aster.into()))
        .or(token(TokenType::Slash.into()))
        .or(token(TokenType::Perc.into()))
        .or(token(TokenType::Hat.into()))
        .or(token(TokenType::Concat.into()))
        .or(token(TokenType::And.into()))
        .or(token(TokenType::Or.into()))
        .or(token(TokenType::Eql.into()))
        .or(token(TokenType::Ne.into()))
        .or(token(TokenType::Less.into()))
        .or(token(TokenType::Le.into()))
        .or(token(TokenType::Greater.into()))
        .or(token(TokenType::Ge.into()));

    op.map(|op| Binop(op))
}

fn unop<Input>() -> impl Parser<Input, Output = Unop>
where
    Input: Stream<Token = Token>,
    <Input as StreamOnce>::Error: ParseError<
        <Input as StreamOnce>::Token,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >,
{
    let op = token(TokenType::Minus.into())
        .or(token(TokenType::Not.into()))
        .or(token(TokenType::Opus.into()));

    op.map(|op| Unop(op))
}
