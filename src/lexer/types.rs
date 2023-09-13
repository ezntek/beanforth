use crate::error::Error;

pub(super) type LexerResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Output, // Period
    Equal,
    Gt,
    Lt,
    EndWord,   // Semicolon
    BeginWord, // Colon
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservedWord {
    If,
    Then,
    Else,
    Do,
    Loop,
}

impl ReservedWord {
    pub fn try_to_string<S: AsRef<str>>(s: S) -> Option<Self> {
        use ReservedWord as R;
        match s.as_ref() {
            "if" => Some(R::If),
            "then" => Some(R::Then),
            "else" => Some(R::Else),
            "do" => Some(R::Do),
            "loop" => Some(R::Loop),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Math(Math),
    Symbol(Character),
    Word(String),
    ReservedWord(ReservedWord),
    Literal(i32),
    Begin,
    End,
}
