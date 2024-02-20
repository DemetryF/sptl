mod declarations;
mod expr;
mod statements;

pub use sptlc_hir::{AssignOp, BinOp, UnOp};
pub use sptlc_hir::{FunRef, VarRef};
pub use sptlc_hir::{Literal, Type};

pub use declarations::*;
pub use expr::*;
pub use statements::*;

pub type THIR<'source> = Vec<Declaration<'source>>;

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
