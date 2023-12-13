use crate::{code_stream::CodeStream, token::TokenKind};

use super::TokenCollector;

pub struct SpecialCollector;

impl<'source> TokenCollector<'source> for SpecialCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>> {
        Self::double(code_stream).or(Self::single(code_stream))
    }
}

impl SpecialCollector {
    pub fn double<'source>(code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>> {
        let value = {
            match code_stream.slice_from_current(2) {
                "+=" => TokenKind::AddAssign,
                "-=" => TokenKind::SubAssign,
                "*=" => TokenKind::MulAssign,
                "/=" => TokenKind::DivAssign,

                ">=" => TokenKind::Ge,
                "<=" => TokenKind::Le,
                "!=" => TokenKind::Ne,
                "==" => TokenKind::Eq,

                "&&" => TokenKind::And,
                "||" => TokenKind::Or,

                "->" => TokenKind::Arrow,

                _ => return None,
            }
        };

        code_stream.skip_n(2);

        Some(value)
    }

    pub fn single<'source>(code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>> {
        let value = {
            match code_stream.current() {
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '=' => TokenKind::Assign,
                '>' => TokenKind::Gt,
                '<' => TokenKind::Lt,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Asterisk,
                '/' => TokenKind::Slash,
                '!' => TokenKind::Not,
                ':' => TokenKind::Colon,

                _ => return None,
            }
        };

        code_stream.skip_n(1);

        Some(value)
    }
}
