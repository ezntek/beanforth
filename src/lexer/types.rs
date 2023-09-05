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

impl Word {
    pub fn from_string(txt: String) -> Option<Self> {
        use Word as W;
        match txt.as_str() {
            "cr" => Some(W::Cr),
            "or" => Some(W::Or),
            "dup" => Some(W::Dup),
            "rot" => Some(W::Rot),
            "mod" => Some(W::Mod),
            "and" => Some(W::And),
            "drop" => Some(W::Drop),
            "swap" => Some(W::Swap),
            "over" => Some(W::Over),
            "emit" => Some(W::Emit),
            "invert" => Some(W::Invert),
            _ => None,
        }
    }

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
