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

pub enum Literal {}
