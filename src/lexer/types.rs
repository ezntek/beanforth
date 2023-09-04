use std::collections::HashMap;

use crate::hash_map;

#[derive(Debug)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Word {
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    Emit,
    Cr,

    // operations
    Mod,
    And,
    Or,
    Invert, // `Not`
}

#[derive(Debug)]
pub enum Symbol {
    Period,
    PeriodDoubleQuotes,
    Equal,
    Gt,
    Lt,
}

#[derive(Debug)]
pub enum Token {
    Math(Math),
    Symbol(Symbol),
    BuiltinWord(Word),
    Literal(i32),
    Eof,
}

impl ToString for Word {
    fn to_string(&self) -> String {
        use Word as W;
        let res = match self {
            W::Cr => "cr",
            W::Or => "or",
            W::Dup => "dup",
            W::Rot => "rot",
            W::Mod => "mod",
            W::And => "and",
            W::Drop => "drop",
            W::Swap => "swap",
            W::Over => "over",
            W::Emit => "emit",
            W::Invert => "invert",
        };
        res.to_owned()
    }
}

impl Word {
    pub fn get_lengths_of_variants() -> HashMap<usize, Vec<Word>> {
        use Word as W;
        return hash_map! {
            2 => vec![W::Cr, W::Or],
            3 => vec![W::Dup, W::Rot, W::Mod, W::And],
            4 => vec![W::Drop, W::Swap, W::Over, W::Emit],
            6 => vec![W::Invert]
        };
    }
}
