pub enum Instruction {
    // jumps
    Goto {
        label: Label,
    },

    Cmp {
        lhs: Atom,
        op: CmpOp,
        rhs: Atom,

        then_label: Label,
        else_label: Option<Label>,
    },

    Call {
        id: FunId,
        args: Vec<Atom>,
    },

    // assignments
    Binary {
        result: Id,

        lhs: Atom,
        op: ArithmOp,
        rhs: Atom,
    },

    Unary {
        result: Id,

        op: UnOp,
        rhs: Atom,
    },

    Copy {
        result: Id,
        value: Atom,
    },
}

pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum CmpOp {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}

pub enum UnOp {
    Neg,
}

pub enum Type {
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

pub struct Id;
pub struct FunId;
pub struct Atom;
pub struct Label;
