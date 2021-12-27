use crate::errors::ScanError;
pub use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl Into<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
        }
    }
}

pub struct Scanner<'source> {
    pub source: &'source str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    // current_index: usize,
    // needs context?
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens = Vec::new();
        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan(&mut self) -> Result<usize, ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eos, "", self.line));
        Ok(self.tokens.len())
    }

    fn scan_token(&mut self) -> Result<(), ScanError> {
        use TokenType::*;
        let c = self.advance()?;
        match c {
            '(' => {
                self.push_token(ParenL);
            }
            ')' => {
                self.push_token(ParenR);
            }
            '{' => {
                self.push_token(BraceL);
            }
            '}' => {
                self.push_token(BraceR);
            }
            '[' => {
                //TODO: multiline string literal....!!!1
                self.push_token(BracketL);
            }
            ']' => {
                self.push_token(BracketR);
            }
            ',' => {
                self.push_token(Comma);
            }
            '.' => {
                self.push_token(Period);
            }
            '-' => {
                // comment: --
                if self.test('-')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.push_token(Minus);
                }
            }
            '+' => {
                self.push_token(Plus);
            }
            '&' => {
                self.push_token(Amp);
            }
            '|' => {
                self.push_token(Bar);
            }
            '%' => {
                self.push_token(Perc);
            }
            '^' => {
                self.push_token(Hat);
            }
            ';' => {
                self.push_token(SemiColon);
            }
            ':' => {
                let tok = if self.test(':')? { DbColon } else { Colon };
                self.push_token(tok);
            }
            '*' => {
                self.push_token(Aster);
            }
            '~' => {
                let tok = if self.test('=')? { Ne } else { Tilda };
                self.push_token(tok);
            }
            '=' => {
                let tok = if self.test('=')? {
                    TokenType::Eq
                } else {
                    Assign
                };
                self.push_token(tok);
            }
            '<' => {
                let tok = if self.test('=')? { Le } else { Less };
                self.push_token(tok);
            }
            '>' => {
                let tok = if self.test('=')? { Ge } else { Greater };
                self.push_token(tok);
            }
            '/' => {
                let tok = if self.test('/')? { IDiv } else { Slash };
                self.push_token(tok);
            }
            '#' => {
                self.push_token(TokenType::Hash);
            }

            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }

            _ => {
                todo!()
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), ScanError> {
        todo!()
    }

    fn number(&mut self) -> Result<(), ScanError> {
        todo!()
    }

    fn name(&mut self) -> Result<(), ScanError> {
        todo!()
    }

    fn advance(&mut self) -> Result<char, ScanError> {
        let c = self.getchar(self.current as usize)?;
        self.current += 1;
        Ok(c)
    }

    fn test(&mut self, expected: char) -> Result<bool, ScanError> {
        if self.is_at_end() {
            return Ok(false);
        }
        let c = self.getchar(self.current)?;
        if c != expected {
            return Ok(false);
        }

        self.current += 1;
        Ok(true)
    }

    fn getchar(&mut self, nth: usize) -> Result<char, ScanError> {
        self.source
            .chars()
            .nth(nth)
            .ok_or_else(|| ScanError::raise())
    }

    fn peek(&mut self) -> Result<char, ScanError> {
        if self.is_at_end() {
            Ok('\0')
        } else {
            self.getchar(self.current)
        }
    }

    fn peek_next(&mut self) -> Result<char, ScanError> {
        if self.current + 1 >= self.source.len() {
            Ok('\0')
        } else {
            self.getchar(self.current + 1)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn push_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
