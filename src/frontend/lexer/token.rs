#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Amp,
    SemiColon,
    LBrack,
    RBrack,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Plus,
    Star,
    Slash,
    Minus,
    Dot,
    Comma,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Eof,
    Illegal(char),
}

#[derive(PartialEq, Debug)]
pub struct Location {
    pub row: u32,
    pub col: u32,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Location,
}
