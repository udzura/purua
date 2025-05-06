// Incremented recursion limit due to compilated Or<> types
// this took some much longer to compile
// see: https://github.com/Marwes/combine/issues/172#issuecomment-401566216
// FIXME: simplify the Or<> types in e.g. binop() or expr()
#![recursion_limit = "1024"]

pub mod errors;
pub mod parser;
pub mod scanner;
pub mod token_type;

pub use crate::scanner::*;
