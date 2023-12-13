use sptlc_ast::Pos;

pub type LexResult<T> = Result<T, LexError>;

#[derive(Debug)]
pub struct LexError {
    pub char: char,
    pub pos: Pos,
}
