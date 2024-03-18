use crate::operators::{BinOp, UnOp};
use crate::{Ident, Type};

pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Self>,
    },
    Call {
        id: Ident<'source>,
        args: Vec<Self>,
    },
    Cast {
        lhs: Box<Self>,
        ty: Type,
    },
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal<'source>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'source> {
    Number {
        value: &'source str,
        postfix: Option<NumberPostfix>,
    },
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberPostfix {
    F64,
    F32,

    I64,
    I32,
    I16,
    I8,

    U64,
    U32,
    U16,
    U8,
}
