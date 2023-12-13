use sptlc_ast::{Literal, NumberType};

use crate::token::TokenKind;
use crate::Lexer;

macro_rules! lexer_test {
    (
        $code:expr;

        [
            $($value:expr),* $(,)?
        ]
    ) => {
        let mut lexer = Lexer::new($code);

        $(
            assert_eq!(
                lexer.next_token().unwrap().kind,
                $value
            );
        )*
    };
}

#[test]
fn numbers() {
    lexer_test![
        "
            384_400     /* common number */
            3.1415      /* double number */
            6.67e-11    /* exponential notation (minus)   */
            6.022e+23   /* exponential notation (plus)    */ 
            1E10        /* exponential notation (no sign) */
            1_f32       /* type prefix */
        ";

        [
            TokenKind::Literal(Literal::Number {
                value: 384400.0,
                postfix: None,
            }),
            TokenKind::Literal(Literal::Number {
                value: 3.1415,
                postfix: None,
            }),
            TokenKind::Literal(Literal::Number {
                value: 6.67e-11,
                postfix: None,
            }),
            TokenKind::Literal(Literal::Number {
                value: 6.022e+23,
                postfix: None,
            }),
            TokenKind::Literal(Literal::Number {
                value: 1e10,
                postfix: None,
            }),
            TokenKind::Literal(Literal::Number {
                value: 1.0,
                postfix: Some(NumberType::F32),
            }),
        ]
    ];
}

#[test]
pub fn bools() {
    lexer_test![
        "true false";

        [
            TokenKind::Literal(Literal::Bool(true)),
            TokenKind::Literal(Literal::Bool(false))
        ]
    ];
}

#[test]
pub fn keywords() {
    lexer_test![
        "continue return const break while else let fn if as";

        [
            TokenKind::Continue,
            TokenKind::Return,
            TokenKind::Const,
            TokenKind::Break,
            TokenKind::While,
            TokenKind::Else,
            TokenKind::Let,
            TokenKind::Fn,
            TokenKind::If,
            TokenKind::As,
        ]
    ];
}

#[test]
pub fn idents() {
    lexer_test![
        "_name12$ Break";

        [
            TokenKind::Ident("_name12$".into()),
            TokenKind::Ident("Break".into())
        ]
    ];
}

#[test]
pub fn specials() {
    lexer_test![
        "
            ;,(){}=
            != >= > <= < ==
            ->
            || && !
            + - * /
        ";

        [
            TokenKind::Semicolon,
            TokenKind::Comma,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::RBrace,
            TokenKind::Assign,

            TokenKind::Ne,
            TokenKind::Ge,
            TokenKind::Gt,
            TokenKind::Le,
            TokenKind::Lt,
            TokenKind::Eq,

            TokenKind::Arrow,

            TokenKind::Or,
            TokenKind::And,
            TokenKind::Not,

            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Asterisk,
            TokenKind::Slash
        ]
    ];
}
