use sptlc_ast::{Literal, Type};

use crate::{code_stream::CodeStream, token::TokenKind};

use super::TokenCollector;

pub struct WordCollector;

impl<'source> TokenCollector<'source> for WordCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>> {
        if !Self::is_word_start(code_stream) {
            return None;
        }

        let buffer = Self::word_literal(code_stream);

        Some(match buffer {
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "const" => TokenKind::Const,
            "break" => TokenKind::Break,
            "while" => TokenKind::While,
            "else" => TokenKind::Else,
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "as" => TokenKind::As,

            "true" => TokenKind::Literal(Literal::Bool(true)),
            "false" => TokenKind::Literal(Literal::Bool(false)),

            "bool" => TokenKind::Type(Type::Bool),
            "f64" => TokenKind::Type(Type::F64),
            "f32" => TokenKind::Type(Type::F32),
            "i64" => TokenKind::Type(Type::I64),
            "i32" => TokenKind::Type(Type::I32),
            "i16" => TokenKind::Type(Type::I16),
            "i8" => TokenKind::Type(Type::I8),
            "u64" => TokenKind::Type(Type::U64),
            "u32" => TokenKind::Type(Type::U32),
            "u16" => TokenKind::Type(Type::U16),
            "u8" => TokenKind::Type(Type::U8),

            id => TokenKind::Ident(id),
        })
    }
}

impl WordCollector {
    fn is_word_start(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_alphabetic()
            || code_stream.check('$')
            || code_stream.check('_')
    }

    fn is_word_char(code_stream: &mut CodeStream) -> bool {
        !code_stream.is_eof()
            && (Self::is_word_start(code_stream) || code_stream.current().is_alphanumeric())
    }

    fn word_literal<'source>(code_stream: &mut CodeStream<'source>) -> &'source str {
        let start = code_stream.index();

        while Self::is_word_char(code_stream) {
            code_stream.next();
        }

        let end = code_stream.index();

        code_stream.slice(start, end)
    }
}
