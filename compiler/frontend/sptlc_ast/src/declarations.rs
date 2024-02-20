use crate::{Block, Expr, Ident, Type};

pub enum Declaration<'source> {
    Functoin(FunctionDeclaration<'source>),
    Constant(ConstantDeclaration<'source>),
}

pub struct FunctionDeclaration<'source> {
    pub id: Ident<'source>,
    pub args: Vec<FunctionArg<'source>>,
    pub return_ty: Type,
    pub body: Block<'source>,
}

pub struct FunctionArg<'source> {
    pub id: Ident<'source>,
    pub ty: Type,
}

pub struct ConstantDeclaration<'source> {
    pub id: Ident<'source>,
    pub ty: Type,
    pub expr: Option<Expr<'source>>,
}
