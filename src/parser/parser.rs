use combine::{many1, token, Parser};

use crate::Token;

use super::stream::TokenStream;

pub fn parse_dummy(stream: TokenStream) -> Result<(), String> {
    let c = Token::new(
        crate::token_type::TokenType::Int,
        "1",
        1,
    );
    let tok = token(c);
    let eof = Token::new(
        crate::token_type::TokenType::Eof,
        "",
        1,
    );
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