mod declarations;
mod expr;
mod statements;

pub use sptlc_ast::{AssignOp, BinOp, Literal, Pos, Type, UnOp};

pub use declarations::*;
pub use expr::*;
pub use statements::*;

pub type HIR<'source> = Vec<Declaration<'source>>;

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
