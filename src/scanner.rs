use crate::errors::ScanError;
pub use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl From<TokenType> for Token {
    fn from(token_type: TokenType) -> Self {
        Self {
            token_type,
            lexeme: String::new(),
            line: 0,
        }
    }
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

impl TryFrom<Token> for String {
    type Error = ScanError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::StringLit => Ok(value.lexeme),
            _ => Err(ScanError::raise()),
        }
    }
}

impl TryFrom<Token> for f64 {
    type Error = ScanError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::Int => value.lexeme.parse::<f64>().map_err(|_| ScanError::raise()),
            TokenType::Float => value.lexeme.parse::<f64>().map_err(|_| ScanError::raise()),
            _ => Err(ScanError::raise()),
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'source> {
    pub source: &'source str,
    pub tokens: Vec<Token>,
    pub comments: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    // current_index: usize,
    // needs context?
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens = Vec::new();
        let comments = Vec::new();
        Self {
            source,
            tokens,
            comments,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<usize, ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
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
                if self.test('.')? {
                    if self.test('.')? {
                        self.push_token(Concat);
                    } else {
                        self.push_token(Dots);
                    }
                } else {
                    self.push_token(Period);
                }
            }
            '-' => {
                // comment: --
                if self.test('-')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                    self.push_comment();
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
                let tok = if self.test('=')? { Eql } else { Assign };
                self.push_token(tok);
            }
            '<' => {
                if self.test('<')? {
                    self.push_token(ShL);
                } else if self.test('=')? {
                    self.push_token(Le);
                } else {
                    self.push_token(Less);
                }
            }
            '>' => {
                if self.test('>')? {
                    self.push_token(ShR);
                } else if self.test('=')? {
                    self.push_token(Ge);
                } else {
                    self.push_token(Greater);
                }
            }
            '/' => {
                let tok = if self.test('/')? { IDiv } else { Slash };
                self.push_token(tok);
            }
            '#' => {
                self.push_token(Opus);
            }

            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
            }
            '\'' => {
                self.string('\'')?;
            }
            '"' => {
                self.string('"')?;
            }

            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.name()?;
                } else {
                    eprintln!("unexpected character: {}", c);
                    return Err(ScanError::raise());
                }
            }
        }
        Ok(())
    }

    fn string(&mut self, quote: char) -> Result<(), ScanError> {
        while self.peek()? != quote && !self.is_at_end() {
            if self.peek()? == '\n' {
                eprintln!("cannot contain linebreak");
                return Err(ScanError::raise());
            }
            self.advance()?;
        }

        if self.is_at_end() {
            eprintln!("Unterminated string");
            return Err(ScanError::raise());
        }

        // The closing quote.
        self.advance()?;
        self.push_token(TokenType::StringLit);

        Ok(())
    }

    fn number(&mut self) -> Result<(), ScanError> {
        let mut is_float = false;
        while is_digit(self.peek()?) {
            self.advance()?;
        }
        if self.peek()? == '.' && is_digit(self.peek_next()?) {
            // Consume the "."
            self.advance()?;

            while is_digit(self.peek()?) {
                self.advance()?;
            }
            is_float = true
        }

        if is_float {
            self.push_token(TokenType::Float);
        } else {
            self.push_token(TokenType::Int);
        }
        Ok(())
    }

    fn name(&mut self) -> Result<(), ScanError> {
        use TokenType::*;

        while is_alphanumeric(self.peek()?) {
            self.advance()?;
        }

        let start = self.start;
        let end = self.current;
        let text = &self.source[start..end];

        let tok = match text {
            "and" => And,
            "break" => Break,
            "do" => Do,
            "else" => Else,
            "elseif" => Elseif,
            "end" => End,
            "false" => False,
            "for" => For,
            "function" => Function,
            "goto" => Goto,
            "if" => If,
            "in" => In,
            "local" => Local,
            "nil" => Nil,
            "not" => Not,
            "or" => Or,
            "repeat" => Repeat,
            "return" => Return,
            "then" => Then,
            "true" => True,
            "until" => Until,
            "while" => While,
            _ => Name,
        };

        self.push_token(tok);
        Ok(())
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

    fn push_comment(&mut self) {
        let lexeme = &self.source[self.start..self.current];
        self.comments
            .push(Token::new(TokenType::Comment, lexeme, self.line));
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
