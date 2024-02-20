mod declarations;
mod expr;
mod operators;
mod pos;
mod statements;

pub use declarations::*;
pub use expr::*;
pub use operators::*;
pub use pos::Pos;
pub use statements::*;

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}

pub struct Ident<'source> {
    pub id: &'source str,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Bool,

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
