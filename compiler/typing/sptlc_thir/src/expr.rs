use sptlc_hir::{FunRef, Literal, VarRef};

use crate::{BinOp, Type, UnOp};

pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        ty: Type,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        ty: Type,
        rhs: Box<Self>,
    },
    Call {
        fun: FunRef<'source>,
        args: Vec<Self>,
    },
    Cast {
        lhs: Box<Self>,
        ty: Type,
    },
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Var(VarRef<'source>),
    Literal(Literal<'source>),
}
