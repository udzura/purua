use std::error::Error;

use purua::scanner::{Scanner, Token, TokenType};

extern crate purua;

#[test]
fn test_scanned_1() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/test_1.lua");
    let mut scanner = Scanner::new(source);
    scanner.scan()?;

    use TokenType::*;
    let expected = vec![
        Token {
            token_type: If,
            lexeme: "if".to_string(),
            line: 2,
        },
        Token {
            token_type: Int,
            lexeme: "1".to_string(),
            line: 2,
        },
        Token {
            token_type: Plus,
            lexeme: "+".to_string(),
            line: 2,
        },
        Token {
            token_type: Int,
            lexeme: "1".to_string(),
            line: 2,
        },
        Token {
            token_type: Less,
            lexeme: "<".to_string(),
            line: 2,
        },
        Token {
            token_type: Int,
            lexeme: "2".to_string(),
            line: 2,
        },
        Token {
            token_type: Do,
            lexeme: "do".to_string(),
            line: 2,
        },
        Token {
            token_type: Name,
            lexeme: "print".to_string(),
            line: 3,
        },
        Token {
            token_type: ParenL,
            lexeme: "(".to_string(),
            line: 3,
        },
        Token {
            token_type: StringLit,
            lexeme: "\"hello, world\"".to_string(),
            line: 3,
        },
        Token {
            token_type: ParenR,
            lexeme: ")".to_string(),
            line: 3,
        },
        Token {
            token_type: Return,
            lexeme: "return".to_string(),
            line: 4,
        },
        Token {
            token_type: Int,
            lexeme: "3".to_string(),
            line: 4,
        },
        Token {
            token_type: End,
            lexeme: "end".to_string(),
            line: 5,
        },
        Token {
            token_type: Eof,
            lexeme: "".to_string(),
            line: 6,
        },
    ];

    assert_eq!(expected, scanner.tokens);
    Ok(())
}

#[test]
fn test_scanned_2() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/test_2.lua");
    let mut scanner = Scanner::new(source);
    scanner.scan()?;

    assert_eq!(157, scanner.tokens.len());
    Ok(())
}

#[test]
fn test_scan_failed_1() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/error_1.lua");
    let mut scanner = Scanner::new(source);

    assert!(scanner.scan().is_err());
    Ok(())
}
