#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Opus = b'#' as u16,
    Perc = b'%' as u16,
    Amp = b'&' as u16,
    ParenL = b'(' as u16,
    ParenR = b')' as u16,
    Aster = b'*' as u16,
    Plus = b'+' as u16,
    Comma = b',' as u16,
    Minus = b'-' as u16,
    Period = b'.' as u16,
    Slash = b'/' as u16,
    Colon = b':' as u16,
    SemiColon = b';' as u16,
    Less = b'<' as u16,
    Assign = b'=' as u16,
    Greater = b'>' as u16,
    BracketL = b'[' as u16,
    BracketR = b']' as u16,
    Hat = b'^' as u16,
    BraceL = b'{' as u16,
    Bar = b'|' as u16,
    BraceR = b'}' as u16,
    Tilda = b'~' as u16,

    And = 257,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    IDiv,
    Concat,
    Dots,
    Eql,
    Ge,
    Le,
    Ne,
    ShL,
    ShR,
    DbColon,
    Eof,

    Float,
    Int,
    Name,
    StringLit,

    Comment,
}
