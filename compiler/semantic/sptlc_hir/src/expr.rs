use std::rc::Rc;

use crate::{BinOp, Literal, Pos, Type, UnOp};

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

pub type FunRef<'source> = Rc<FunData<'source>>;
pub type VarRef<'source> = Rc<VarData<'source>>;

pub struct FunData<'source> {
    pub id: &'source str,
    pub args: Vec<FunctionArg<'source>>,
    pub declaration_pos: Pos,
    pub return_ty: Option<Type>,
}

pub struct FunctionArg<'source> {
    pub data: VarRef<'source>,
    pub ty: Type,
}

pub struct VarData<'source> {
    pub id: &'source str,
    pub declaration_pos: Pos,
    pub ty: Type,
}
