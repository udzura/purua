use crate::Token;

use combine::{error::StreamError, stream::ResetStream, ParseError, Positioned, StreamOnce};

macro_rules! debug_error_msg {
    ($($msg:tt)*) => {
        if std::env::var("DEBUG").is_ok() {
            eprintln!($($msg)*);
        }
    };
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    pub input: Vec<Token>,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStreamError {
    pub position: usize,
}

impl StreamError<Token, Vec<Token>> for TokenStreamError {
    fn unexpected_token(token: Token) -> Self {
        debug_error_msg!("unexpected token: {:?}", token);
        TokenStreamError {
            position: token.line,
        }
    }

    fn unexpected_range(token: Vec<Token>) -> Self {
        todo!()
    }

    fn unexpected_format<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }

    fn expected_token(token: Token) -> Self {
        debug_error_msg!("expected token: {:?}", token);
        TokenStreamError {
            position: token.line,
        }
    }

    fn expected_range(token: Vec<Token>) -> Self {
        todo!()
    }

    fn expected_format<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }

    fn message_token(token: Token) -> Self {
        todo!()
    }

    fn message_range(token: Vec<Token>) -> Self {
        todo!()
    }

    fn message_format<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }

    fn is_unexpected_end_of_input(&self) -> bool {
        todo!()
    }

    fn into_other<T>(self) -> T
    where
        T: StreamError<Token, Vec<Token>>,
    {
        todo!()
    }
}

impl ParseError<Token, Vec<Token>, usize> for TokenStreamError {
    type StreamError = Self;
    fn from_error(position: usize, _message: TokenStreamError) -> Self {
        TokenStreamError { position }
    }

    fn empty(position: usize) -> Self {
        debug_error_msg!("reached empty");
        TokenStreamError { position }
    }

    fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    fn add(&mut self, err: Self::StreamError) {
        debug_error_msg!("added error: {:?}", err);
    }

    fn set_expected<F>(self_: &mut combine::error::Tracked<Self>, info: Self::StreamError, f: F)
    where
        F: FnOnce(&mut combine::error::Tracked<Self>),
    {
        todo!()
    }

    fn is_unexpected_end_of_input(&self) -> bool {
        todo!()
    }

    fn into_other<T>(self) -> T
    where
        T: ParseError<Token, Vec<Token>, usize>,
    {
        todo!()
    }
}

impl TokenStreamError {
    pub fn empty(position: usize) -> Self {
        TokenStreamError { position }
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        for token in iter {
            self.input.push(token);
        }
    }
}

impl StreamOnce for TokenStream {
    type Token = Token;
    type Position = usize;
    type Range = Vec<Token>;
    type Error = TokenStreamError;

    fn uncons(&mut self) -> Result<Self::Token, TokenStreamError> {
        if self.position >= self.input.len() {
            Err(TokenStreamError::empty(self.position))
        } else {
            let token = self.input[self.position].clone();
            self.position += 1;
            Ok(token)
        }
    }

    fn is_partial(&self) -> bool {
        false
    }
}

impl ResetStream for TokenStream {
    type Checkpoint = usize;

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), TokenStreamError> {
        self.position = checkpoint;
        Ok(())
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.position
    }
}

impl Positioned for TokenStream {
    fn position(&self) -> Self::Position {
        self.position
    }
}

impl TokenStream {
    pub const fn new(input: Vec<Token>) -> Self {
        TokenStream { input, position: 0 }
    }
}
