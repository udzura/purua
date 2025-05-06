//
// The Complete Syntax of Lua 5.1:
//   From... https://www.lua.org/manual/5.1/manual.html#8
//
// chunk ::= {stat [`;´]} [laststat [`;´]]
// block ::= chunk
// stat ::=  varlist `=´ explist |
// 	 functioncall |
// 	 do block end |
// 	 while exp do block end |
// 	 repeat block until exp |
// 	 if exp then block {elseif exp then block} [else block] end |
// 	 for Name `=´ exp `,´ exp [`,´ exp] do block end |
// 	 for namelist in explist do block end |
// 	 function funcname funcbody |
// 	 local function Name funcbody |
// 	 local namelist [`=´ explist]
// laststat ::= return [explist] | break
// funcname ::= Name {`.´ Name} [`:´ Name]
// varlist ::= var {`,´ var}
// var ::=  Name | prefixexp `[´ exp `]´ | prefixexp `.´ Name
// namelist ::= Name {`,´ Name}
// explist ::= {exp `,´} exp
// exp ::=  nil | false | true | Number | String | `...´ | function |
// 	 prefixexp | tableconstructor | exp binop exp | unop exp
// prefixexp ::= var | functioncall | `(´ exp `)´
// functioncall ::=  prefixexp args | prefixexp `:´ Name args
// args ::=  `(´ [explist] `)´ | tableconstructor | String
// function ::= function funcbody
// funcbody ::= `(´ [parlist] `)´ block end
// parlist ::= namelist [`,´ `...´] | `...´
// tableconstructor ::= `{´ [fieldlist] `}´
// fieldlist ::= field {fieldsep field} [fieldsep]
// field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp
// fieldsep ::= `,´ | `;´
// binop ::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ |
// 	 `<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ |
// 	 and | or
// unop ::= `-´ | not | `#´
//
use crate::Token;

#[derive(Debug, Clone)]
pub struct Chunk(pub Vec<Stat>, pub Option<LastStat>);

#[derive(Debug, Clone)]
pub struct Block(pub Chunk);

#[derive(Debug, Clone)]
pub enum Stat {
    Assign(VarList, ExprList),
    FunctionCall(FunctionCall),
    Do(Block),
    While(Box<Expr>, Block),
    Repeat(Box<Expr>, Block),
    If(Box<Expr>, Block, Vec<(Box<Expr>, Block)>, Option<Block>),
    For(Token, Box<Expr>, Box<Expr>, Option<Box<Expr>>, Block),
    ForIn(NameList, ExprList, Block),
    Function(FuncName, FuncBody),
    LocalFunction(Token, FuncBody),
    LocalDeclVar(NameList, Option<ExprList>),
}

#[derive(Debug, Clone)]
pub enum LastStat {
    Return(Option<ExprList>),
    Break,
}

#[derive(Debug, Clone)]
pub struct FuncName(pub Vec<Token>, pub Option<Token>);

#[derive(Debug, Clone)]
pub struct VarList(pub Vec<Var>);

#[derive(Debug, Clone)]
pub enum Var {
    VarName(Token),
    VarIdx(PrefixExp, Box<Expr>),
    VarMember(PrefixExp, Token),
}

#[derive(Debug, Clone)]
pub struct NameList(pub Vec<Token>);

#[derive(Debug, Clone)]
pub struct ExprList(pub Vec<Expr>);

#[derive(Debug, Clone)]
pub enum Expr {
    Nil,
    False,
    True,
    Number(f64),
    String(String),
    Dots,
    Function(Function),
    PrefixExp(PrefixExp),
    TableConstructor(TableConstructor),
    Binop(Box<Expr>, Binop, Box<Expr>),
    Unop(Unop, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum PrefixExp {
    PrefixVar(Box<Var>),
    PrefixCall(FunctionCall),
    PrefixParen(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct FunctionCall(pub Box<PrefixExp>, pub Option<Token>, pub Args);

#[derive(Debug, Clone)]
pub enum Args {
    ArgsNone,
    ArgsList(ExprList),
    ArgsTable(TableConstructor),
    ArgsString(String),
}

#[derive(Debug, Clone)]
pub struct Function(pub FuncBody);

#[derive(Debug, Clone)]
pub struct FuncBody(pub ParamList, pub Block);

#[derive(Debug, Clone)]
pub struct ParamList(pub NameList, pub bool);

#[derive(Debug, Clone)]
pub struct TableConstructor(pub FieldList);

#[derive(Debug, Clone)]
pub struct FieldList(pub Vec<Field>);

#[derive(Debug, Clone)]
pub enum Field {
    AssignIdx(Box<Expr>, Box<Expr>),
    AssignName(Token, Box<Expr>),
    UniExp(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Fieldsep;

#[derive(Debug, Clone)]
pub struct Binop(pub Token);

#[derive(Debug, Clone)]
pub struct Unop(pub Token);
