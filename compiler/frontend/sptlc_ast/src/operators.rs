#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// '||'
    Or,
    /// '&&'
    And,
    /// '=='
    Eq,
    /// '!='
    Ne,
    /// '>='
    Ge,
    /// '<='
    Le,
    /// '>'
    Gt,
    /// '<'
    Lt,
    /// '+'
    Add,
    /// '-'
    Sub,
    /// '*'
    Mul,
    /// '/'
    Div,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    /// '-'
    Neg,
    /// '!'
    Not,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    /// '='
    Assign,
    /// '+='
    AddAssign,
    /// '-='
    SubAssign,
    /// '*='
    MulAssign,
    /// '/='
    DivAssign,
}
