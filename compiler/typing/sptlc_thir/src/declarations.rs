use crate::{Block, Expr, FunRef, VarRef};

pub enum Declaration<'source> {
    Function(FunctionDeclaration<'source>),
    Constant(ConstantDeclaration<'source>),
}

pub struct FunctionDeclaration<'source> {
    pub data: FunRef<'source>,
    pub body: Block<'source>,
}

pub struct ConstantDeclaration<'source> {
    pub data: VarRef<'source>,
    pub value: Expr<'source>,
}
