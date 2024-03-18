use sptlc_ast::{Literal, NumberPostfix};

use crate::{CodeStream, TokenValue};

use super::TokenCollector;

const RADIX_PREFIX_LENGTH: usize = 2;

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_collect<'source>(
        &mut self,
        code_stream: &mut CodeStream<'source>,
    ) -> Option<TokenValue<'source>> {
        if !Self::is_digit(code_stream, 10) {
            return None;
        }

        let value = Self::value(code_stream);
        let postfix = Self::postfix(code_stream);

        Some(TokenValue::Literal(Literal::Number { value, postfix }))
    }
}

impl NumberCollector {
    pub fn is_digit(code_stream: &CodeStream, radix: u32) -> bool {
        code_stream.current().is_digit(radix)
    }

    pub fn value<'source>(code_stream: &mut CodeStream<'source>) -> &'source str {
        let start = code_stream.index();

        match code_stream.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => Self::prefixed(code_stream, 2),
            "0o" => Self::prefixed(code_stream, 8),
            "0x" => Self::prefixed(code_stream, 16),
            _ => Self::common_number(code_stream),
        };

        let end = code_stream.index();

        code_stream.slice(start, end)
    }

    pub fn prefixed(code_stream: &mut CodeStream, radix: u32) {
        code_stream.skip(RADIX_PREFIX_LENGTH);
        Self::number_literal(code_stream, radix);
    }

    pub fn common_number(code_stream: &mut CodeStream) {
        Self::number_literal(code_stream, 10);

        Self::fraction(code_stream);
        Self::exponential_part(code_stream);
    }

    pub fn fraction(code_stream: &mut CodeStream) {
        if code_stream.check('.') {
            code_stream.next_ch();

            Self::number_literal(code_stream, 10);
        }
    }

    pub fn exponential_part(code_stream: &mut CodeStream) {
        if code_stream.check('e') || code_stream.check('E') {
            code_stream.next_ch();

            if code_stream.check('-') || code_stream.check('+') {
                code_stream.next_ch();
            }

            Self::number_literal(code_stream, 10);
        }
    }

    fn number_literal(code_stream: &mut CodeStream, radix: u32) {
        while !code_stream.is_eof()
            && (Self::is_digit(code_stream, radix) || code_stream.check('_'))
        {
            code_stream.next_ch();
        }
    }

    fn postfix(code_stream: &mut CodeStream) -> Option<NumberPostfix> {
        Self::postfix3ch(code_stream).or_else(|| Self::postfix2ch(code_stream))
    }

    fn postfix3ch(code_stream: &mut CodeStream) -> Option<NumberPostfix> {
        Some(match code_stream.slice_from_current(3) {
            "f64" => NumberPostfix::F64,
            "f32" => NumberPostfix::F32,
            "u64" => NumberPostfix::U64,
            "u32" => NumberPostfix::U32,
            "u16" => NumberPostfix::U16,
            "i64" => NumberPostfix::I64,
            "i32" => NumberPostfix::I32,
            "i16" => NumberPostfix::I16,

            _ => return None,
        })
    }

    fn postfix2ch(code_stream: &mut CodeStream) -> Option<NumberPostfix> {
        Some(match code_stream.slice_from_current(2) {
            "i8" => NumberPostfix::I8,
            "u8" => NumberPostfix::U8,
            _ => return None,
        })
    }
}
