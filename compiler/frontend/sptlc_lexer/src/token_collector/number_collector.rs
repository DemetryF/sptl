use parse_int::parse;
use sptlc_ast::{Literal, NumberType};

use crate::code_stream::CodeStream;
use crate::token::TokenKind;

use super::TokenCollector;

const RADIX_PREFIX_LEN: usize = 2;

pub struct NumberCollector;

impl<'source> TokenCollector<'source> for NumberCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>> {
        if !Self::is_number_start(code_stream) {
            return None;
        }

        let start = code_stream.index();

        match code_stream.slice_from_current(RADIX_PREFIX_LEN) {
            "0b" => Self::eat_prefixed(code_stream, 2),
            "0o" => Self::eat_prefixed(code_stream, 8),
            "0x" => Self::eat_prefixed(code_stream, 16),

            _ => Self::eat_unprefixed(code_stream),
        }

        let end = code_stream.index();

        let buffer = code_stream.slice(start, end);
        let value = parse::<f64>(buffer).unwrap();

        let postfix = Self::postfix(code_stream);

        Some(TokenKind::Literal(Literal::Number { value, postfix }))
    }
}

impl NumberCollector {
    pub fn is_number_start(code_stream: &mut CodeStream) -> bool {
        code_stream.current().is_ascii_digit()
    }

    /// eats prefixed number like '0b101' or '0x42F'
    pub fn eat_prefixed(code_stream: &mut CodeStream, radix: u32) {
        code_stream.skip_n(RADIX_PREFIX_LEN);

        Self::eat_digits(code_stream, radix);
    }

    /// eats unprefixed number like '0.42_e2'
    pub fn eat_unprefixed(code_stream: &mut CodeStream) {
        Self::eat_digits(code_stream, 10);

        Self::eat_fraction_part(code_stream);
        Self::eat_exponent_part(code_stream);
    }

    pub fn eat_digits(code_stream: &mut CodeStream, radix: u32) {
        while Self::is_digit(code_stream, radix) {
            code_stream.next();
        }
    }

    pub fn eat_fraction_part(code_stream: &mut CodeStream) {
        if code_stream.check('.') {
            code_stream.next();

            Self::eat_digits(code_stream, 10);
        }
    }

    pub fn eat_exponent_part(code_stream: &mut CodeStream) {
        if code_stream.try_consume('e') || code_stream.try_consume('E') {
            let _ = code_stream.try_consume('+') || code_stream.try_consume('-');

            Self::eat_digits(code_stream, 10);
        }
    }

    pub fn is_digit(code_stream: &mut CodeStream, radix: u32) -> bool {
        !code_stream.is_eof() && (code_stream.current().is_digit(radix) || code_stream.check('_'))
    }

    pub fn postfix(code_stream: &mut CodeStream) -> Option<NumberType> {
        use NumberType::*;

        match code_stream.slice_from_current(2) {
            "u8" => return Some(U8),
            "i8" => return Some(I8),

            _ => {}
        };

        match code_stream.slice_from_current(3) {
            "i16" => return Some(I16),
            "i32" => return Some(I32),
            "i64" => return Some(I64),

            "u16" => return Some(U16),
            "u32" => return Some(U32),
            "u64" => return Some(U64),

            "f32" => return Some(F32),
            "f64" => return Some(F64),

            _ => None,
        }
    }
}
