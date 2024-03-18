use std::fmt::{self, Display};

use sptlc_ast::{Literal, Pos, Type};

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenValue<'source> {
    // keywords
    Break,
    Continue,
    Const,
    Else,
    Fn,
    If,
    Let,
    Return,
    While,
    As,

    // special chars
    /// ';'
    Semicolon,
    /// '+='
    AddAssign,
    /// '-='
    SubAssign,
    /// '*='
    MulAssign,
    /// '/='
    DivAssign,
    /// ','
    Comma,
    /// '->'
    Arrow,
    /// '{'
    LBrace,
    /// '}'
    RBrace,
    /// '('
    LParen,
    /// ')'
    RParen,
    /// '='
    Assign,
    /// '||'
    Or,
    /// '&&'
    And,
    /// '!='
    Ne,
    /// '=='
    Eq,
    /// '>='
    Ge,
    /// '>'
    Gt,
    /// '<='
    Le,
    /// '<'
    Lt,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '!'
    Not,

    // other
    Literal(Literal<'source>),
    Id(&'source str),
    Type(Type),

    EOF,
}

impl<'source> Display for TokenValue<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            TokenValue::Break => "break",
            TokenValue::Continue => "continue",
            TokenValue::Const => "const",
            TokenValue::Else => "else",
            TokenValue::Fn => "fn",
            TokenValue::If => "if",
            TokenValue::Let => "let",
            TokenValue::Return => "return",
            TokenValue::While => "while",
            TokenValue::As => "as",

            TokenValue::Arrow => "->",
            TokenValue::Semicolon => ";",
            TokenValue::Comma => ",",
            TokenValue::LBrace => "{",
            TokenValue::RBrace => "}",
            TokenValue::LParen => "(",
            TokenValue::RParen => ")",
            TokenValue::Assign => "=",
            TokenValue::Or => "|",
            TokenValue::And => "&",
            TokenValue::Ne => "!=",
            TokenValue::Eq => "==",
            TokenValue::Ge => ">=",
            TokenValue::Gt => ">",
            TokenValue::Le => "<=",
            TokenValue::Lt => "<",
            TokenValue::Plus => "+",
            TokenValue::AddAssign => "+=",
            TokenValue::Minus => "-",
            TokenValue::SubAssign => "-=",
            TokenValue::Star => "*",
            TokenValue::MulAssign => "*=",
            TokenValue::Slash => "/",
            TokenValue::DivAssign => "/=",
            TokenValue::Not => "!",

            TokenValue::Literal(literal) => {
                return write!(f, "{literal}");
            }

            TokenValue::Id(id) => id,

            TokenValue::Type(ty) => match ty {
                Type::Bool => "bool",
                Type::F64 => "f64",
                Type::F32 => "f32",
                Type::I64 => "i64",
                Type::I32 => "i32",
                Type::I16 => "i16",
                Type::I8 => "i8",
                Type::U64 => "u64",
                Type::U32 => "u32",
                Type::U16 => "u16",
                Type::U8 => "u8",
            },

            TokenValue::EOF => "\\0",
        };

        write!(f, "{value}")
    }
}
