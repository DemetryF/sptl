use sptlc_ast::{Literal, Pos, Type};

pub struct Token<'source> {
    pub kind: TokenKind<'source>,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind<'source> {
    Continue,
    Return,
    Const,
    Break,
    While,
    Else,
    Let,
    If,
    Fn,
    As,

    /// ';'
    Semicolon,
    /// '('
    LParen,
    /// ')'
    RParen,
    /// '{'
    LBrace,
    /// '}'
    RBrace,
    /// '->'
    Arrow,
    /// ':'
    Colon,
    /// ','
    Comma,

    /// '='
    Assign,
    /// '+='
    AddAssign,
    /// '-='
    SubAssign,
    /// '*='
    MulAssign,
    /// '/='
    DivAssign,

    /// '||'
    Or,
    /// '&&'
    And,
    /// '!'
    Not,
    /// '=='
    Eq,
    /// '!='
    Ne,
    /// '>='
    Ge,
    /// '<='
    Le,
    /// '>'
    Gt,
    /// '<'
    Lt,

    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Asterisk,
    /// '/'
    Slash,

    Ident(&'source str),
    Literal(Literal),
    Type(Type),
    EOF,
}
