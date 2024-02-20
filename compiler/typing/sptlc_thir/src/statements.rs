use sptlc_hir::{AssignOp, VarRef};

use crate::{Block, Expr};

pub enum Statement<'source> {
    If(IfStatement<'source>),
    While(WhileStatement<'source>),
    Expr(ExprStatement<'source>),
    Return(ReturnStatement<'source>),
    Break,
    Continue,
}

pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub then_branch: Block<'source>,
    pub elseif_branches: Vec<ElseIfBranch<'source>>,
}

pub struct ElseIfBranch<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub enum ExprStatement<'source> {
    Assign {
        var: VarRef<'source>,
        op: AssignOp,
        value: Expr<'source>,
    },
    Expr(Expr<'source>),
}

pub struct ReturnStatement<'source> {
    pub expr: Option<Expr<'source>>,
}
