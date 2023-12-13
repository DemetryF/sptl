use crate::{
    operators::{BinOp, UnOp},
    Ident,
};

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
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number {
        value: f64,
        postfix: Option<NumberType>,
    },
    Bool(bool),
}

#[derive(Debug, PartialEq)]

pub enum NumberType {
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
