use std::fmt::Display;

use crate::{Literal, NumberPostfix};

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number { value, postfix } => {
                write!(f, "{value}")?;

                if let Some(postfix) = postfix {
                    write!(f, "{postfix}")?;
                }
            }
            Literal::Bool(bool) => write!(f, "{bool}")?,
        }

        Ok(())
    }
}

impl Display for NumberPostfix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = {
            match self {
                NumberPostfix::F64 => "f64",
                NumberPostfix::F32 => "f32",
                NumberPostfix::I64 => "i64",
                NumberPostfix::I32 => "i32",
                NumberPostfix::I16 => "i16",
                NumberPostfix::I8 => "i8",
                NumberPostfix::U64 => "u64",
                NumberPostfix::U32 => "u32",
                NumberPostfix::U16 => "u16",
                NumberPostfix::U8 => "u8",
            }
        };

        write!(f, "{str}")
    }
}
