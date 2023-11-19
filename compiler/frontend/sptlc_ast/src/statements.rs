use crate::{operators::AssignOp, Block, Expr, Ident, Type};

pub enum Statement<'source> {
    If(IfStatement<'source>),
    While(WhileStatement<'source>),
    Let(LetStatement<'source>),
    Expr(ExprStatement<'source>),
    Return(ReturnStatement<'source>),
    Break(BreakStatement),
    Continue(ContinueStatement),
}

pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub then_branch: Block<'source>,

    pub elseif_branches: Vec<ElseIfBranch<'source>>,
    pub else_branch: Option<Block<'source>>,
}

pub struct ElseIfBranch<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub struct LetStatement<'source> {
    pub id: Ident<'source>,
    pub ty: Type,
    pub expr: Option<Expr<'source>>,
}

pub enum ExprStatement<'source> {
    Assign {
        id: Ident<'source>,
        op: AssignOp,
        value: Expr<'source>,
    },
    Expr(Expr<'source>),
}

pub struct ReturnStatement<'source> {
    pub expr: Option<Expr<'source>>,
}

pub struct BreakStatement;
pub struct ContinueStatement;
